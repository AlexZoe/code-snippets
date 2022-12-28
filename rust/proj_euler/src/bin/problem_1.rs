fn main() {
    let sum: u64 = (1..1000)
        .map(|x| if x % 3 == 0 || x % 5 == 0 { x } else { 0 })
        .sum();
    println!("Sum is {}", sum);
}
