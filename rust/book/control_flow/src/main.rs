fn main() {
    if_func();
    loop_func();
    for_func();
}


fn if_func() {
    // condition has to be boolean
    if 3 > 0 { // curly brackets are required
        println!("true");
    }
    let number = if false { 3 } else { 4 };
    println!("number is {}", number);
}


fn loop_func() {
    // infinite loop (same as while(true)
    let mut a = 0;
    loop {
        a += 1;
        if a > 5 {
            break;
        }
    }
    println!("finished");
}


fn for_func() {
    let array = [10, 20, 30, 40];

    // use array builtin function
    for i in array.iter() {
        println!("val at idx: {}", i);
    }

    // range
    for i in 0..3 {
        println!("idx: {}", i);
    }

    // range in reverse
    for i in (0..3).rev() {
        println!("idx: {}", i);
    }
}
