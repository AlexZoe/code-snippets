fn main() {
    let x = 5;
    let x = x + 1; // new variable 'x' overlaying (shadowing) previous x
    let x = x * 2;
    println!("x is {}", x);

    let my_string = "hello there";
    let my_string = my_string.len(); // works also for variables with different type
    println!("string is: {}", my_string);
}
