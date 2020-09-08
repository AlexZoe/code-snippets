enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // create new heap allocated heap data type
    // need to specify type
    let mut a:  Vec<i32> = Vec::new();
    a.push(32);

    // use macro initializer for vector
    let mut b = vec!(21, 11);
    b.push(3);
    println!("{:?}", b);

    let third: &i32 = &b[2];

    match b.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterate over immutable vector elements
    for i in &b {
        println!("{}", i);
    }

    // iterate over mutable vector elements
    for i in &mut b {
        *i += 3;
        println!("{}", i);
    }

    // push different enum types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
