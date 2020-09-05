fn main() {
    let my_string = String::from("hi there");
    // function requires immutable slice
    let word = get_first_word(&my_string[..]);

    // this will cause an error since my_string is immutable
//    my_string.clear();

    // using a slice is safer than accessing the string by indices (string might be mutable)
    println!("{}", word);

    other_function();
}


fn get_first_word(input_str: &str) -> &str {
    let bytes = input_str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // look for space
        if item == b' ' {
            // return slice of string which is bound to input string
            return &input_str[..i];
        }
    }
    &input_str[..]
}


fn other_function() {
    let a = [1, 2, 3, 4, 5];

    // slices work with other types too
    let array_slice = &a[1..3];
    for i in array_slice {
        println!("{}", i);
    }
}
