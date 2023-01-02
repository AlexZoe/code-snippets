mod prime;
use prime::factorizer;

fn main() {
    let factorizer = factorizer::PrimeFactorizer::new(600851475143);
    let factors: Vec<u64> = factorizer.into_iter().collect();
    println!("factors: {:?}\n", factors);

    let factorizer = factorizer::PrimeFactorizer::new(13195);
    let factors: Vec<u64> = factorizer.into_iter().collect();
    println!("factors: {:?}\n", factors);

    let factorizer = factorizer::PrimeFactorizer::new(9);
    let factors: Vec<u64> = factorizer.into_iter().collect();
    println!("factors: {:?}\n", factors);
}
