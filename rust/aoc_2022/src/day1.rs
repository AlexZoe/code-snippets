pub struct CalorieSummer<'a> {
    input: std::str::Lines<'a>,
}

impl<'a> CalorieSummer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.lines(),
        }
    }
}

impl<'a> Iterator for CalorieSummer<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let mut total = 0;
        let mut iters = 0;
        while let Some(next) = self.input.next() {
            iters += 1;
            match next.parse::<u64>() {
                Ok(i) => {
                    total += i;
                }
                _ => {
                    break;
                }
            }
        }
        return if iters > 0 { Some(total) } else { None };
    }
}

#[cfg(test)]
mod tests {
    use super::super::file;
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn empty_string_for_non_existing_file() {
        assert_eq!(String::from(""), file::get_string_from_file(""));
    }

    #[test]
    fn return_highest_for_single_number() {
        let mut summer = CalorieSummer::new("1000");
        assert_eq!(1000, summer.next().unwrap());
    }

    #[test]
    fn return_highest_sum_in_list() {
        let string = file::get_string_from_file("assets/day1_sample.txt");
        let summer = CalorieSummer::new(&string);
        let map: BinaryHeap<u64> = summer.collect();

        assert_eq!(24000, *map.peek().unwrap());
    }
}
