mod day1;
mod day2;
mod day3;
mod day4;
mod file;

use day4::Overlap;

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

fn day3_part_one() {
    let string = file::get_string_from_file("assets/day3_real_input.txt");
    let sum: u64 = day3::DuplicateItemValueCounter::new(&string).sum();
    println!("score is: {}", sum);
}

fn day3_part_two() {
    let string = file::get_string_from_file("assets/day3_real_input.txt");
    let sum: u64 = day3::BadgeValueFinder::new(&string).sum();
    println!("score is: {}", sum);
}

fn day4_part_one() {
    let string = file::get_string_from_file("assets/day4_real_input.txt");
    println!(
        "number full overlapping sections: {}",
        day4::OverlapFinder::new(&string).full_overlap()
    );
}

fn day4_part_two() {
    let string = file::get_string_from_file("assets/day4_real_input.txt");
    println!(
        "number partial overlapping sections: {}",
        day4::OverlapFinder::new(&string).partial_overlap()
    );
}

fn main() {
    println!("day1");
    day1_part_one();
    day1_part_two();

    println!("\nday2");
    day2_part_one();
    day2_part_two();

    println!("\nday3");
    day3_part_one();
    day3_part_two();

    println!("\nday4");
    day4_part_one();
    day4_part_two();
}
