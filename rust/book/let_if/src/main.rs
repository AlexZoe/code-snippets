fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }

    // Short form for matches
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("none");
    }
}
