mod prime;
use prime::factorizer;

fn main() {
    let mut number: u64 = 2*3*5*7*11*13*17*19;
    println!("start number: {}\n", number);

    let mut again = 1;
    while again == 1 {
        again = 0;
        for i in (1..21).rev() {
            if number % i != 0 {
                let factorizer = factorizer::PrimeFactorizer::new(i);
                let factors: Vec<u64> = factorizer.into_iter().collect();
                number *= factors[0];
                again = 1;
            }
        }
    }
    println!("final num: {}\n", number);
}
