fn main() {
    let x: u32 = 15u32; // variables can have a type suffix
    let y = 16u8; // type u8
    // The following will not work since x and y have different types
//    let z = x + y;
    println!("x is {}", x);
    println!("y is {}", y);
    let long_var = 1_000_000; // can also have underscores for better readability
    println!("long is {}", long_var);

    // create compound: tuple
    let tup = (5, true, 1.2);
    // destructure compound in single variables
    let (a, b, c) = tup;
    println!("a, b, c: {}, {}, {}", a, b, c)
}
