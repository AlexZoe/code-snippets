use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;

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

pub fn get_string_from_file(file: &str) -> String {
    let mut s = String::new();
    let _ = match File::open(file) {
        Ok(mut f) => f.read_to_string(&mut s),
        _ => Ok(0),
    };
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_for_non_existing_file() {
        assert_eq!(String::from(""), get_string_from_file(""));
    }

    #[test]
    fn highest_is_0_for_empty_string() {
        assert_eq!(0, find_hightest_cals_of_elf(""));
    }

    #[test]
    fn return_highest_for_single_number() {
        let mut summer = CalorieSummer::new("1000");
        assert_eq!(1000, summer.next().unwrap());
    }

    #[test]
    fn return_highest_sum_in_list() {
        let string = get_string_from_file("assets/day1_sample.txt");
        let mut summer = CalorieSummer::new(&string);
        let map: BinaryHeap<u64> = summer.collect();

        assert_eq!(24000, *map.peek().unwrap());
    }
}
