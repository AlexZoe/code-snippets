#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    amount: u64,
    from: u64,
    to: u64,
}

pub struct InstructionList<'a> {
    instructions: std::str::Lines<'a>,
}

impl<'a> InstructionList<'a> {
    pub fn new(list: &'a str) -> Self {
        Self {
            instructions: list.lines(),
        }
    }
}

impl<'a> Iterator for InstructionList<'a> {
    type Item = Operation;
    fn next(&mut self) -> Option<Self::Item> {
        match self.instructions.next() {
            Some(line) => {
                let items: Vec<&str> = line.split(' ').collect();
                Some(Operation {
                    amount: items[1].parse::<u64>().unwrap(),
                    from: items[3].parse::<u64>().unwrap(),
                    to: items[5].parse::<u64>().unwrap(),
                })
            }
            None => None,
        }
    }
}

pub struct SupplyStack {
    supplies: Vec<Vec<char>>,
}

impl SupplyStack {
    pub fn new(supplies: Vec<Vec<char>>) -> Self {
        Self { supplies }
    }

    pub fn apply_instructions(&mut self, instructions: InstructionList) {
        for op in instructions {
            for _ in 0..op.amount {
                let len: usize = self.supplies[(op.from - 1) as usize].len();
                let mut tail = self.supplies[(op.from - 1) as usize].split_off(len - 1);
                self.supplies[(op.to - 1) as usize].append(&mut tail);
            }
        }
    }

    pub fn apply_instructions_bulk(&mut self, instructions: InstructionList) {
        for op in instructions {
            let len: usize = self.supplies[(op.from - 1) as usize].len();
            let mut tail =
                self.supplies[(op.from - 1) as usize].split_off(len - (op.amount) as usize);
            self.supplies[(op.to - 1) as usize].append(&mut tail);
        }
    }

    pub fn get_top(&self) -> Vec<char> {
        self.supplies
            .iter()
            .map(|x| x.last().unwrap().clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::file;
    use super::*;

    #[test]
    fn get_single_instruction() {
        let mut list = InstructionList::new("move 1 from 2 to 1");
        assert_eq!(
            Operation {
                amount: 1,
                from: 2,
                to: 1
            },
            list.next().unwrap()
        );
    }

    #[test]
    fn get_initial_supply_stack() {
        let mut stack = SupplyStack::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(vec!['N', 'D', 'P'], stack.get_top());
    }

    #[test]
    fn get_supply_stack_after_instructions() {
        let mut stack = SupplyStack::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        let mut list = InstructionList::new("move 1 from 2 to 1");

        stack.apply_instructions(list);
        assert_eq!(vec!['D', 'C', 'P'], stack.get_top());
    }

    #[test]
    fn get_supply_stack_after_sample_instructions() {
        let mut stack = SupplyStack::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        let string = file::get_string_from_file("assets/day5_sample.txt");
        let mut list = InstructionList::new(&string);

        stack.apply_instructions(list);
        assert_eq!(vec!['C', 'M', 'Z'], stack.get_top());
    }

    #[test]
    fn get_supply_stack_after_sample_instructions_with_bulk() {
        let mut stack = SupplyStack::new(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        let string = file::get_string_from_file("assets/day5_sample.txt");
        let mut list = InstructionList::new(&string);

        stack.apply_instructions_bulk(list);
        assert_eq!(vec!['M', 'C', 'D'], stack.get_top());
    }
}
