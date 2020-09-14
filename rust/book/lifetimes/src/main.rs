// struct with lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let next_result;
    {
        let another_string = String::from("hey");
        next_result = longest(string1.as_str(), another_string.as_str());
    }
    // the following won't work since another_string is no longer valid
//    println!("The longest string is {}", next_result);


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // struct is only valid if it does not live longer than the reference it holds
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// explicitly annotate lifetime
// the return value has the lifetime of the shorter one of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
