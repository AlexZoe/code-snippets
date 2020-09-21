use std::ops::Deref;

struct MyBox<T>(T);


impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


pub fn own_box_type() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


pub fn my_deref_call() {
    let m = MyBox::new(String::from("Rust"));
    // deref coercion:
    // deref is called on box which returns the string
    // then deref is called on String which returns a string slice reference
    // what happens in the background is: hello(&(*m)[..]);
    // deref coercion happens at compile time; deref is called as many times as required
    hello(&m);
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}
