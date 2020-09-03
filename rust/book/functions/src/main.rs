use rand::Rng;

fn main() {
    print_int(3);
    println!("rand is: {}", get_rand_int());

    let y = {
        let x = 3; // this is a statement
        x + 1 // by omitting ';' it turns into an expression returning a value
    };
    println!("y is: {}", y);
}

// type of function parameter has to be defined
fn print_int(x: u32) {
    println!("x is: {}", x);
}


fn get_rand_int() -> u32 {
    // expressions work also in functions (missing ';')
    rand::thread_rng().gen_range(1, 101)
}


fn get_five() -> u32 {
    // 'return' statement works as well
    return 5;
}
