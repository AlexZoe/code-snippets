mod day1;
mod day2;
mod file;

use std::collections::BinaryHeap;

fn day1_part_one() {
    let string = file::get_string_from_file("assets/day1_real_input.txt");
    let map: BinaryHeap<u64> = day1::CalorieSummer::new(&string).collect();
    println!("highest: {}", *map.peek().unwrap());
}

fn day1_part_two() {
    let string = file::get_string_from_file("assets/day1_real_input.txt");
    let map: BinaryHeap<u64> = day1::CalorieSummer::new(&string).collect();
    println!(
        "highest three combined: {}",
        map.into_iter().take(3).sum::<u64>()
    );
}

fn day2_part_one() {
    let string = file::get_string_from_file("assets/day2_real_input.txt");
    let sum: u64 = day2::Janken::new(&string, day2::get_score_by_result).sum();
    println!("score is: {}", sum);
}

fn day2_part_two() {
    let string = file::get_string_from_file("assets/day2_real_input.txt");
    let sum: u64 = day2::Janken::new(&string, day2::get_score_by_type).sum();
    println!("score for updated rule is: {}", sum);
}

fn main() {
    day1_part_one();
    day1_part_two();

    day2_part_one();
    day2_part_two();
}
