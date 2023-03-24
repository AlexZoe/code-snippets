pub struct DuplicateItemValueCounter<'a> {
    input: std::str::Lines<'a>,
}

impl<'a> DuplicateItemValueCounter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.lines(),
        }
    }

    fn get_duplicate_value(&self, line: &str) -> u64 {
        let (first, second) = line.split_at(line.len() / 2);

        let item_list_first = get_seen_character_list(first);
        let item_list_second = get_seen_character_list(second);

        let mut idx = 0;
        item_list_first
            .into_iter()
            .zip(&item_list_second)
            .map(|(lhs, rhs)| {
                idx += 1;
                if *rhs == lhs {
                    rhs * idx
                } else {
                    0
                }
            })
            .sum()
    }
}

impl<'a> Iterator for DuplicateItemValueCounter<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            Some(line) => Some(self.get_duplicate_value(line)),
            _ => None,
        }
    }
}

pub struct BadgeValueFinder<'a> {
    input: std::str::Lines<'a>,
}

impl<'a> BadgeValueFinder<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.lines(),
        }
    }
}

impl<'a> Iterator for BadgeValueFinder<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let mut list = vec![0; 52];
        for _ in 0..3 {
            match self.input.next() {
                Some(line) => {
                    let tmp = get_seen_character_list(line);
                    list = list.iter().zip(tmp).map(|(lhs, rhs)| lhs + rhs).collect();
                }
                None => break,
            }
        }

        let mut idx = 0;
        let result: u64 = list
            .iter()
            .map(|x| {
                idx += 1;
                if *x == 3 {
                    idx
                } else {
                    0
                }
            })
            .sum();

        if result > 0 {
            Some(result)
        } else {
            None
        }
    }
}

fn map_char_to_index(input: &char) -> u8 {
    let input_val = *input as u8;
    match input_val as u8 {
        97..=122 => input_val - 97,
        65..=90 => input_val - 39,
        _ => panic!("non ascii character\n"),
    }
}

fn get_seen_character_list(input: &str) -> Vec<u64> {
    let mut list = vec![0; 52];
    for c in input.chars() {
        let idx = map_char_to_index(&c);
        if list[idx as usize] == 0 {
            list[idx as usize] = 1;
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use super::super::file;
    use super::*;

    #[test]
    fn return_zero_for_no_duplicate() {
        let counter = DuplicateItemValueCounter::new("ab");
        let value: u64 = counter.sum();
        assert_eq!(0, value);
    }

    #[test]
    fn return_value_for_duplicate() {
        let counter = DuplicateItemValueCounter::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        let value: u64 = counter.sum();
        assert_eq!(16, value);
    }

    #[test]
    fn return_value_count_for_test_input() {
        let string = file::get_string_from_file("assets/day3_sample.txt");
        let counter = DuplicateItemValueCounter::new(&string);
        let value: u64 = counter.sum();
        assert_eq!(157, value);
    }

    #[test]
    fn return_value_of_badges_for_test_input() {
        let string = file::get_string_from_file("assets/day3_sample.txt");
        let counter = BadgeValueFinder::new(&string);
        let value: u64 = counter.sum();
        assert_eq!(70, value);
    }
}
