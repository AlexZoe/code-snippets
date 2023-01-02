use std::collections::VecDeque;

fn main() {
    let mut highest: u64 = 0;
    let mut first: u64 = 0;
    let mut second: u64 = 0;

    for i in (100..999).rev() {
        for j in (100..999).rev() {
            let number = i * j;
            let mut tmp = i * j;
            let mut digits: Vec<u64> = vec![];
            let mut digits_rev = VecDeque::new();
            while tmp > 0 {
                digits.push(tmp % 10);
                digits_rev.push_front(tmp % 10);
                tmp /= 10;
            }
            let matching = digits
                .iter()
                .zip(&digits_rev)
                .filter(|&(a, b)| a == b)
                .count();
            if matching == digits.len() {
                if number > highest {
                    highest = number;
                    first = i;
                    second = j;
                }
            }
        }
    }
    println!("highest match {} ({}, {})", highest, first, second);
}
