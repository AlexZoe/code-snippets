use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete,
    character::complete::digit1,
    combinator::opt,
    multi::many1,
    IResult,
};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Add,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StressModifier {
    pub op: Operation,
    pub constant: Option<u64>,
}

#[derive(Debug, PartialEq)]
struct ThrowTarget {
    if_true: usize,
    if_false: usize,
    check: u64,
}

impl ThrowTarget {
    pub fn throw_to(&self, stress: u64) -> usize {
        if stress % self.check == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<u64>,
    stress_modifier: StressModifier,
    target: ThrowTarget,
    seen_items: u64,
    stress_relief: StressModifier,
}

impl Monkey {
    fn throw_next(&mut self) -> Option<(u64, usize)> {
        let item = self.items.pop_front();
        match item {
            Some(item) => {
                self.seen_items += 1;
                let rhs = self.stress_modifier.constant.unwrap_or(item);
                let item = stress_update(&item, &self.stress_modifier.op, &rhs);
                let item = stress_update(
                    &item,
                    &self.stress_relief.op,
                    &self.stress_relief.constant.unwrap(),
                );
                let target = self.target.throw_to(item);
                Some((item, target))
            }
            None => None,
        }
    }

    fn get_seen_items(&self) -> u64 {
        self.seen_items
    }

    fn set_stress_relief_modifier(&mut self, modifier: StressModifier) {
        self.stress_relief = modifier;
    }
}

fn stress_update(lhs: &u64, op: &Operation, rhs: &u64) -> u64 {
    match op {
        Operation::Add => lhs + rhs,
        Operation::Mul => lhs * rhs,
        Operation::Div => lhs / rhs,
        Operation::Mod => lhs % rhs,
    }
}

pub struct Circus {
    monkeys: Vec<Monkey>,
    round: u64,
}

impl Circus {
    pub fn new(input: &str, stress_relief: Option<StressModifier>) -> Self {
        let mut circus = Self {
            monkeys: Vec::new(),
            round: 0,
        };

        let mut input = input;
        while input.len() > 0 {
            let (remainder, monkey) = parse_monkey(input).expect("valid monkey");
            circus.monkeys.push(monkey);
            input = remainder;
        }

        let modifier = match stress_relief {
            Some(modifier) => modifier,
            None => StressModifier {
                op: Operation::Mod,
                constant: Some(
                    circus
                        .monkeys
                        .iter()
                        .map(|x| x.target.check)
                        .product::<u64>(),
                ),
            },
        };

        for i in 0..circus.monkeys.len() {
            circus.monkeys[i].set_stress_relief_modifier(modifier.clone());
        }

        circus
    }

    pub fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some((item, target)) = self.monkeys[i].throw_next() {
                self.monkeys[target].items.push_back(item);
            }
        }
        self.round += 1;
    }

    pub fn get_scores(&self) -> Vec<u64> {
        self.monkeys.iter().map(|x| x.get_seen_items()).collect()
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = start_of_next_line(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = many1(parse_items)(input)?;
    let (input, _) = start_of_next_line(input)?;
    let (input, stress_modifier) = parse_stress_modifier(input)?;
    let (input, _) = start_of_next_line(input)?;
    let (input, target) = parse_throw_target(input)?;

    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;

    Ok((
        input,
        Monkey {
            items: items.into_iter().collect::<VecDeque<_>>(),
            stress_modifier,
            target,
            seen_items: 0,
            stress_relief: StressModifier {
                op: Operation::Div,
                constant: Some(1),
            },
        },
    ))
}

fn start_of_next_line(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_while(|c: char| c != '\n')(input)?;
    Ok(take_while(|c: char| c.is_whitespace())(input)?)
}

fn parse_items(input: &str) -> IResult<&str, u64> {
    let (input, _) = opt(tag(", "))(input)?;
    let (input, num) = complete::u64(input)?;
    Ok((input, num))
}

fn parse_stress_modifier(input: &str) -> IResult<&str, StressModifier> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, op) = take_while(|c: char| !c.is_whitespace())(input)?;

    let op = match op {
        "+" => Operation::Add,
        "*" => Operation::Mul,
        _ => unreachable!(),
    };

    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;
    let (input, maybe_const) = alt((tag("old"), digit1))(input)?;

    let constant = match maybe_const {
        "old" => None,
        constant => Some(constant.parse::<u64>().expect("parsable number")),
    };

    Ok((input, StressModifier { op, constant }))
}

fn parse_throw_target(input: &str) -> IResult<&str, ThrowTarget> {
    let (input, _) = take_while(|c: char| !c.is_numeric())(input)?;
    let (input, check) = complete::u64(input)?;
    let (input, _) = start_of_next_line(input)?;
    let (input, _) = take_while(|c: char| !c.is_numeric())(input)?;
    let (input, if_true) = complete::u64(input)?;
    let (input, _) = start_of_next_line(input)?;
    let (input, _) = take_while(|c: char| !c.is_numeric())(input)?;
    let (input, if_false) = complete::u64(input)?;

    Ok((
        input,
        ThrowTarget {
            if_true: if_true as usize,
            if_false: if_false as usize,
            check,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn parse_monkey_from_str() {
        let (_, monkey) = parse_monkey(
            "Monkey 0:\n\
                Starting items: 79, 98\n\
                Operation: new = old * 19\n\
                Test: divisible by 23\n\
                If true: throw to monkey 2\n\
                If false: throw to monkey 3",
        )
        .expect("valid monkey");

        let ref_monkey = Monkey {
            items: [79, 98].into(),
            stress_modifier: StressModifier {
                op: Operation::Mul,
                constant: Some(19),
            },
            target: ThrowTarget {
                if_true: 2,
                if_false: 3,
                check: 23,
            },
            seen_items: 0,
            stress_relief: StressModifier {
                op: Operation::Div,
                constant: Some(1),
            },
        };

        println!("monkey: {:?}", monkey);
        println!("ref monkey: {:?}", ref_monkey);

        assert_eq!(ref_monkey, monkey);
    }

    #[test]
    fn construct_circus() {
        let input = fs::read_to_string("./assets/day11_sample.txt").expect("Read ./test_input");
        let mut circus = Circus::new(
            input.as_str(),
            Some(StressModifier {
                op: Operation::Div,
                constant: Some(3),
            }),
        );

        for (i, monkey) in circus.monkeys.iter().enumerate() {
            println!("monkey[{}]: {:?}", i, monkey.items);
        }

        for i in 0..20 {
            println!("\nround: {}", i);
            circus.round();
            for (idx, monkey) in circus.monkeys.iter().enumerate() {
                println!("monkey[{}]: {:?}", idx, monkey.items);
            }
        }

        let vec: Vec<u64> = circus.monkeys.iter().map(|x| x.get_seen_items()).collect();
        println!("seen: {:?}", vec);
    }
}
