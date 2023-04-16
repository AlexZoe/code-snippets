use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete,
    character::complete::newline,
    combinator::opt,
    IResult,
};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Add(i64),
}

#[derive(Debug)]
struct State {
    reg: i64,
    cycle_count: u64,
}

#[derive(Debug)]
pub struct ProcessorState {
    reg: i64,
    cycle_count: u64,
    callstack: Vec<State>,
}

impl ProcessorState {
    pub fn new() -> Self {
        Self {
            reg: 1,
            cycle_count: 1,
            callstack: Vec::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {
                self.callstack.push(State {
                    reg: self.reg,
                    cycle_count: self.cycle_count,
                });
                self.cycle_count += 1;
            }

            Instruction::Add(val) => {
                self.callstack.push(State {
                    reg: self.reg,
                    cycle_count: self.cycle_count,
                });
                self.cycle_count += 2;
                self.reg += val;
            }
        }
    }

    pub fn get_reg_value_at_cycle(&self, cycle: u64) -> i64 {
        let val = self
            .callstack
            .iter()
            .filter(|&entry| entry.cycle_count <= cycle)
            .last();

        if let Some(val) = val {
            val.reg
        } else {
            self.reg
        }
    }

    pub fn draw_crt(&self) {
        let crt_width = 40;
        let crt_height = 6;

        for y in 0..crt_height {
            for x in 0..crt_width {
                let current_cycle = y * crt_width + x + 1;
                let reg = self.get_reg_value_at_cycle(current_cycle);
                print!(
                    "{}",
                    if (reg - 1) <= x as i64 && (reg + 1) >= x as i64 {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!("");
        }
    }
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut input = input;
    let mut instructions = Vec::new();

    while input.len() > 0 {
        let (remainder, instr) = parse_instruction(input).expect("parse instruction");
        instructions.push(instr);
        input = remainder;
    }
    instructions
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instr) = alt((tag("noop"), tag("addx")))(input)?;
    let (input, instr) = match instr {
        "noop" => (input, Instruction::Noop),
        "addx" => {
            let (input, _) = tag(" ")(input)?;
            let (input, val) = complete::i64(input)?;
            (input, Instruction::Add(val))
        }

        _ => unreachable!(),
    };
    let (input, _) = take_while1(|c: char| c.is_whitespace())(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((input, instr))
}

#[cfg(test)]
mod test {
    use super::super::file;
    use super::*;

    #[test]
    fn expect_one_on_start() {
        assert_eq!(1, ProcessorState::new().get_reg_value_at_cycle(0));
    }

    #[test]
    fn expect_value_changed_after_add() {
        let mut state = ProcessorState::new();
        state.execute(Instruction::Add(10));
        assert_eq!(1, state.get_reg_value_at_cycle(0));
        assert_eq!(11, state.get_reg_value_at_cycle(3));
    }

    #[test]
    fn parse_sample_input() {
        let instructions =
            parse_instructions(file::get_string_from_file("assets/day10_sample.txt").as_str());
        let mut state = ProcessorState::new();

        for instr in instructions {
            state.execute(instr);
        }

        let mut score = 0;

        for cycle in (20..260).step_by(40) {
            let tmp = state.get_reg_value_at_cycle(cycle);
            println!("reg for cycle {} is {}", cycle, tmp);
            score += tmp * cycle as i64;
        }

        assert_eq!(13140, score);
    }
}
