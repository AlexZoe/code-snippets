#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // perform an action depending on the matching enum type
    match coin {
        // different matches
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // print the state as additional action for quarters
            println!("Quarter from {:?}", state);
            25
        }
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // do nothing if option does not hold a value
        None => None, // None matching rule is mandatory for Option type
        // add 1 to the value inside the option
        Some(i) => Some(i + 1)
    }
}

fn some_u8_val(x: u8) {
    match x {
        1 => println!("one"),
        3 => println!("three"),
        // catch all rule for remaining values; has to be add the end
        _ => ()
    }
}

fn main() {
    // Insert value in to 'Option' type
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
