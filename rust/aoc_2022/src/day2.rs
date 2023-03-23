pub struct Janken<'a, F>
where
    F: Fn(&str, &str) -> u64,
{
    input: std::str::Lines<'a>,
    matcher: F,
}

impl<'a, F> Janken<'a, F>
where
    F: Fn(&str, &str) -> u64,
{
    pub fn new(input: &'a str, matcher: F) -> Self {
        Self {
            input: input.lines(),
            matcher,
        }
    }
}

impl<'a, F> Iterator for Janken<'a, F>
where
    F: Fn(&str, &str) -> u64,
{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            Some(line) => {
                let mut split = line.split(" ");
                let lhs = split.next().unwrap();
                let rhs = split.next().unwrap();

                return Some((self.matcher)(lhs, rhs));
            }
            _ => {
                return None;
            }
        }
    }
}

enum JankenResultScore {
    WIN = 6,
    DRAW = 3,
    LOSS = 0,
}

enum JankenTypeScore {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

pub fn get_score_by_result(lhs: &str, rhs: &str) -> u64 {
    let mut score = 0;
    match rhs {
        "X" => {
            score += JankenTypeScore::ROCK as u64;
            match lhs {
                "A" => score += JankenResultScore::DRAW as u64,

                "B" => score += JankenResultScore::LOSS as u64,
                _ => score += JankenResultScore::WIN as u64,
            }
        }
        "Y" => {
            score += JankenTypeScore::PAPER as u64;
            match lhs {
                "A" => score += JankenResultScore::WIN as u64,

                "B" => score += JankenResultScore::DRAW as u64,

                _ => score += JankenResultScore::LOSS as u64,
            }
        }
        _ => {
            score += JankenTypeScore::SCISSORS as u64;
            match lhs {
                "A" => score += JankenResultScore::LOSS as u64,
                "B" => score += JankenResultScore::WIN as u64,

                _ => score += JankenResultScore::DRAW as u64,
            }
        }
    }
    score
}

pub fn get_score_by_type(lhs: &str, rhs: &str) -> u64 {
    let mut score = 0;
    match rhs {
        "X" => match lhs {
            "A" => score += JankenTypeScore::SCISSORS as u64,

            "B" => score += JankenTypeScore::ROCK as u64,

            _ => score += JankenTypeScore::PAPER as u64,
        },
        "Y" => {
            score += JankenResultScore::DRAW as u64;
            match lhs {
                "A" => score += JankenTypeScore::ROCK as u64,
                "B" => score += JankenTypeScore::PAPER as u64,
                _ => score += JankenTypeScore::SCISSORS as u64,
            }
        }
        _ => {
            score += JankenResultScore::WIN as u64;
            match lhs {
                "A" => score += JankenTypeScore::PAPER as u64,
                "B" => score += JankenTypeScore::SCISSORS as u64,

                _ => score += JankenTypeScore::ROCK as u64,
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::super::file;
    use super::*;

    #[test]
    fn return_1_for_paper_vs_rock() {
        let mut janken = Janken::new("B X", get_score_by_result);
        assert_eq!(1, janken.next().unwrap());
    }

    #[test]
    fn return_sum_for_test_input_part_one() {
        let string = file::get_string_from_file("assets/day2_sample.txt");
        let janken = Janken::new(&string, get_score_by_result);
        let sum: u64 = janken.sum();

        assert_eq!(15, sum);
    }

    #[test]
    fn return_sum_for_test_input_part_two() {
        let string = file::get_string_from_file("assets/day2_sample.txt");
        let janken = Janken::new(&string, get_score_by_type);
        let sum: u64 = janken.sum();

        assert_eq!(12, sum);
    }
}
