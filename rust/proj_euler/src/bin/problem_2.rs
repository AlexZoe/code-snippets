use std::mem;

struct Fib {
    first: u64,
    second: u64
}

impl Fib {
    fn new() -> Self {
        Self { first: 1, second: 1}
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        mem::swap(&mut self.first, &mut self.second);
        self.second += self.first;
        Some(self.first)
    }
}

fn main() {
    let fib = Fib::new();
    let result: Vec<u64> = fib.take_while(|x| *x < 4_000_000).collect();

    let fib = Fib::new();
    let sum: u64 = fib.take_while(|x| *x < 4_000_000).map(|x| if x % 2 == 0 {x} else {0}).sum();

    println!("{:?}", result);
    println!("{:?}", sum);
}
