fn add_one(x: i32) -> i32 {
    x + 1
}


// function which takes a function pointer where the provided function
// takes a single i32 argument and returns an i32 value
// 'fn' is a type whereas 'Fn' is a trait (see below)
fn do_twice_with_function(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


// function which takes Box argument containing a type which implements the 'Fn' trait
// trait argument has to be placed in Box since the size of the function arguments
// have to be known at compile time
fn do_twice_with_closure_or_function(f: Box<dyn Fn(i32) -> i32>, arg: i32) -> i32 {
    f(arg) + f(arg)
}


fn main() {
    let answer = do_twice_with_function(add_one, 5);
    println!("The answer is: {}", answer);

    // create new box with closure
    let my_closure_box = Box::new(|x: i32| -> i32 {
        x + 1
    });
    let answer = do_twice_with_closure_or_function(my_closure_box, 6);
    println!("The answer is: {}", answer);

    // create new box with function
    let my_function_box = Box::new(add_one);
    let answer = do_twice_with_closure_or_function(my_function_box, 7);
    println!("The answer is: {}", answer);
}
