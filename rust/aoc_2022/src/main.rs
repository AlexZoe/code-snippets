mod day1;

use std::collections::BinaryHeap;

fn day1_part_one() {
    let string = day1::get_string_from_file("assets/day1_real_input.txt");
    let map: BinaryHeap<u64> = day1::CalorieSummer::new(&string).collect();
    println!("highest: {}", *map.peek().unwrap());
}

fn day1_part_two() {
    let string = day1::get_string_from_file("assets/day1_real_input.txt");
    let map: BinaryHeap<u64> = day1::CalorieSummer::new(&string).collect();
    println!(
        "highest three combined: {}",
        map.into_iter().take(3).sum::<u64>()
    );
}

fn main() {
    day1_part_one();
    day1_part_two();
}
