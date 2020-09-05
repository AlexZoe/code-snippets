fn main() {
    let mut s = String::from("hi there");
    s.push_str(", how are you");
    println!("{}", s);

    // this is a move operation
    let s2 = s;

    // s is no longer valid at this point
//    println!("{}", s);

    // deep copy
    let s3 = s2.clone();

    // s3 gets moved to function is no longer valid after it returns
    take_ownership(s3);

    // s2 still valid since returned by function
    let s2 = take_and_give_back(s2);
    println!("{}", s2);
}


fn take_ownership(some_string: String) {
    println!("{}", some_string);
}


fn take_and_give_back(mut my_string: String) -> String {
    my_string.push_str(" some more");
    my_string
}
