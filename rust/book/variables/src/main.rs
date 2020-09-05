fn main() {
    let x = 5;
    let mut y = 5;
    println!("Value of x is: {}", x);
    println!("Value of y is: {}", y);
//    x = 6; // Cannot assign to immutable variable
    y = 6;
    println!("Value of y is: {}", y);
}
