fn main() {
    // new empty string
    let mut s = String::new();

    // string slice
    let data = "initial contents";

    // convert to string
    let s = data.to_string();

    // convert data directly to string
    let s = "initial contents".to_string();

    // same as above
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s2 = String::from("this ");
    // append strings (requires string slice)
    s2 = s2 + &s;
    println!("{}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // append string by pushing to the end
    let mut s3 = String::from("that ");
    s3.push_str(&s);

    // print char values which may comprise of 1 or more byte each
    let new_string = String::from("こんにちは");
    for c in new_string.chars() {
        println!("{}", c);
    }

    // get individual bytes of string
    for c in new_string.bytes() {
        println!("{}", c);
    }
}


