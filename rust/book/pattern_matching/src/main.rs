fn main() {
    or_matching();
    range_matching();
    destructing_match();
    ignore_match();
    ignore_multiple();
    match_guard();
    match_bindings();
}


fn or_matching() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        // matches anything; can be used to ignore certain values
        _ => println!("anything"),
    }
}


fn range_matching() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}


struct Point2D {
    x: i32,
    y: i32,
}

fn destructing_match() {
    let p = Point2D { x: 0, y: 7 };

    // split struct fields into individual variables
    let Point2D { x, y } = p;
    println!("x: {}", x);
    println!("y: {}", y);

    // match specific values for fields
    match p {
        Point2D { x, y: 0 } => println!("On the x axis at {}", x),
        Point2D { x: 0, y } => println!("On the y axis at {}", y),
        Point2D { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


fn ignore_match() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}


fn ignore_multiple() {
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        // '..' allows to ignore multiple values without having to use '_' for
        // each one individually
        Point3D { x, .. } => println!("x is {}", x),
    }
}


fn match_guard() {
    let num = Some(4);

    match num {
        // use additional conditions to specify match
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}


fn match_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // create new variable which binds the result of the range match
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            // result is not bound to a variable an thus cannot be used
            println!("Found an id in another range")
        }
        // create new variable which binds result (same as id: id)
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
