mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod file;

use day4::Overlap;

use clap::Parser;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CmdOptions {
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

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

fn day5_part_one() {
    let string = file::get_string_from_file("assets/day5_real_input.txt");
    let instructions = day5::InstructionList::new(&string);

    let mut stack = day5::SupplyStack::new(vec![
        vec!['W', 'B', 'D', 'N', 'C', 'F', 'J'],
        vec!['P', 'Z', 'V', 'Q', 'L', 'S', 'T'],
        vec!['P', 'Z', 'B', 'G', 'J', 'T'],
        vec!['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C'],
        vec!['G', 'V', 'B', 'J', 'S'],
        vec!['P', 'S', 'Q'],
        vec!['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N'],
        vec!['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R'],
        vec!['V', 'D', 'T', 'R'],
    ]);

    stack.apply_instructions(instructions);
    println!(
        "top of stack is: {}",
        stack.get_top().iter().cloned().collect::<String>()
    );
}

fn day5_part_two() {
    let string = file::get_string_from_file("assets/day5_real_input.txt");
    let instructions = day5::InstructionList::new(&string);

    let mut stack = day5::SupplyStack::new(vec![
        vec!['W', 'B', 'D', 'N', 'C', 'F', 'J'],
        vec!['P', 'Z', 'V', 'Q', 'L', 'S', 'T'],
        vec!['P', 'Z', 'B', 'G', 'J', 'T'],
        vec!['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C'],
        vec!['G', 'V', 'B', 'J', 'S'],
        vec!['P', 'S', 'Q'],
        vec!['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N'],
        vec!['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R'],
        vec!['V', 'D', 'T', 'R'],
    ]);

    stack.apply_instructions_bulk(instructions);
    println!(
        "top of stack is: {}",
        stack.get_top().iter().cloned().collect::<String>()
    );
}

fn day6_part_one() {
    let string = file::get_string_from_file("assets/day6_real_input.txt");
    println!(
        "start position is: {}",
        day6::find_start_of_sequence_position(&string, 4).unwrap()
    );
}

fn day6_part_two() {
    let string = file::get_string_from_file("assets/day6_real_input.txt");
    println!(
        "start position is: {}",
        day6::find_start_of_sequence_position(&string, 14).unwrap()
    );
}

fn day7_part_one() {
    let string = file::get_string_from_file("assets/day7_real_input.txt");
    let fs = day7::FileSystem::new(&string);
    let mut sorted: Vec<usize> = fs.dir_size_list();
    sorted.sort();

    let max: usize = 100_000;
    println!(
        "size is: {}",
        sorted.iter().take_while(|x| **x <= max).sum::<usize>()
    );
}

fn day7_part_two() {
    let string = file::get_string_from_file("assets/day7_real_input.txt");
    let fs = day7::FileSystem::new(&string);
    let mut sorted: Vec<usize> = fs.dir_size_list();
    sorted.sort();
    let root_size = sorted.iter().rev().take(1).next().unwrap();
    let min_size = 30_000_000 - (70_000_000 - root_size);

    println!(
        "size is: {}",
        sorted.iter().filter(|x| **x >= min_size).next().unwrap()
    );
}

fn day8_part_one() {
    let string = file::get_string_from_file("assets/day8_real_input.txt");
    println!(
        "visible fields: {}",
        day8::Grid::new(&string).count_visible()
    );
}

fn day8_part_two() {
    let string = file::get_string_from_file("assets/day8_real_input.txt");
    println!(
        "scenic score: {}",
        day8::Grid::new(&string).highest_scenic_view_score()
    );
}

fn day9_part_one() {
    let instructions = day9::parse_walk_instructions(
        file::get_string_from_file("assets/day9_real_input.txt").as_str(),
    );
    let mut rope_bridge = day9::RopeBridge::new(1);

    for inst in instructions {
        rope_bridge.walk(inst);
    }
    println!("visited places: {}", rope_bridge.visited_places());
}

fn day9_part_two() {
    let instructions = day9::parse_walk_instructions(
        file::get_string_from_file("assets/day9_real_input.txt").as_str(),
    );
    let mut rope_bridge = day9::RopeBridge::new(9);

    for inst in instructions {
        rope_bridge.walk(inst);
    }
    println!("visited places: {}", rope_bridge.visited_places());
}

fn day10_part_one() {
    let instructions = day10::parse_instructions(
        file::get_string_from_file("assets/day10_real_input.txt").as_str(),
    );
    let mut state = day10::ProcessorState::new();

    for instr in instructions {
        state.execute(instr);
    }

    let mut score = 0;

    for cycle in (20..260).step_by(40) {
        let tmp = state.get_reg_value_at_cycle(cycle);
        score += tmp * cycle as i64;
    }
    println!("score is: {}", score);
}

fn day10_part_two() {
    let instructions = day10::parse_instructions(
        file::get_string_from_file("assets/day10_real_input.txt").as_str(),
    );
    let mut state = day10::ProcessorState::new();

    for instr in instructions {
        state.execute(instr);
    }

    state.draw_crt();
}

fn day11_part_one() {
    let input = fs::read_to_string("./assets/day11_real_input.txt").expect("Read input");
    let mut circus = day11::Circus::new(
        input.as_str(),
        Some(day11::StressModifier {
            op: day11::Operation::Div,
            constant: Some(3),
        }),
    );

    for _ in 0..20 {
        circus.round();
    }

    let mut vec: Vec<u64> = circus.get_scores();
    vec.sort();
    println!("score: {:?}", vec[vec.len() - 1] * vec[vec.len() - 2]);
}

fn day11_part_two() {
    let input = fs::read_to_string("./assets/day11_real_input.txt").expect("Read input");
    let mut circus = day11::Circus::new(input.as_str(), None);

    for _ in 0..10000 {
        circus.round();
    }

    let mut vec: Vec<u64> = circus.get_scores();
    vec.sort();
    println!("score: {}", vec[vec.len() - 1] * vec[vec.len() - 2]);
}

fn day12_part_one() {
    let input = fs::read_to_string("./assets/day12_real_input.txt").expect("Read ./test_input");
    let mut map = day12::Map::new(&input);
    println!("distance: {}", map.search(false).unwrap());
}

fn day12_part_two() {
    let input = fs::read_to_string("./assets/day12_real_input.txt").expect("Read ./test_input");
    let mut map = day12::Map::new(&input);
    println!(
        "distance: {}",
        map.inverse_search_to_elevation(1, false).unwrap()
    );
}

fn main() {
    let args = CmdOptions::parse();

    match args.day {
        1 => {
            println!("day1");
            day1_part_one();
            day1_part_two();
        }
        2 => {
            println!("\nday2");
            day2_part_one();
            day2_part_two();
        }
        3 => {
            println!("\nday3");
            day3_part_one();
            day3_part_two();
        }
        4 => {
            println!("\nday4");
            day4_part_one();
            day4_part_two();
        }
        5 => {
            println!("\nday5");
            day5_part_one();
            day5_part_two();
        }

        6 => {
            println!("\nday6");
            day6_part_one();
            day6_part_two();
        }
        7 => {
            println!("\nday7");
            day7_part_one();
            day7_part_two();
        }
        8 => {
            println!("\nday8");
            day8_part_one();
            day8_part_two();
        }
        9 => {
            println!("\nday9");
            day9_part_one();
            day9_part_two();
        }
        10 => {
            println!("\nday10");
            day10_part_one();
            day10_part_two();
        }
        11 => {
            println!("\nday11:");
            day11_part_one();
            day11_part_two();
        }
        12 => {
            println!("\nday12:");
            day12_part_one();
            day12_part_two();
        }
        _ => {}
    }
}
