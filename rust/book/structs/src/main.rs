struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,  // optional coma here
}


// struct without field names
struct Color (i32, i32, i32);   // requires semicolon here


fn main() {

    // either whole instance is mutable or not
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // copy remaining fields from user1
    };
}


fn create_new_user(email: String, username: String) -> User {
    User {
        email, // if name matches field name can be left out
        username,
        active: true,
        sign_in_count: 1
    }
}
