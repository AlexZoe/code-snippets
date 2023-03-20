fn naive() -> u64 {
    (1..101).sum::<u64>().pow(2) - (1..101).map(|x: u64| x.pow(2)).sum::<u64>()
}

fn elegant() -> u64 {
    let square_sum = (((100 * (100 + 1)) / 2) as u64).pow(2);
    let sum_squares = (100 * (100 + 1) * ((2 * 100) + 1)) / 6;
    square_sum - sum_squares
}

fn main() {
    assert!(naive() == elegant(), "result differs\n");

    println!("result is {}\n", elegant());
}
