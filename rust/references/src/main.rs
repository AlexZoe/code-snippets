fn main() {
    let mut my_string = String::from("hello");

    let len = get_length(&my_string);

    println!("len of \"{}\" is {}", my_string, len);

    change_string(&mut my_string);

    println!("{}", my_string);
    example();
}


fn get_length(my_string: &String) -> usize {
    my_string.len()
}


fn change_string(my_string: &mut String) {
    my_string.push_str("some more");
}


fn example() {
    let mut a = String::from("this");
    // okay to have multiple inmutable references at the same time
    let b = &a;
    let c = &a;

    // only okay to have a mutable reference since inmutable ones are no longer used
    let d = &mut a;
    d.push_str(" and that");
    println!("{}", d);
}
