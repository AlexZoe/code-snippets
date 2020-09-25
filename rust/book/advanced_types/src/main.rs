fn main() {
    // create alias for types
    type uint32_t = u32;
    let a: uint32_t = 3;

    // create alias for closure storage
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
}


// The type '!' indicates that the function never returns
fn bar() -> ! {
    panic!();

    // this is also of type '!' since the loop never returns
    loop {
    }
}


// This function only takes arguments where its size is known at compile time
fn generic_1<T>(t: T) {
}

// equivalent to the function above
fn generic_2<T: Sized>(t: T) {
}


// this function takes a parameter of dynamic size which may not be known at compile time
// types of dynamic size have to be placed behind a pointer (&T)
fn generic_3<T: ?Sized>(t: &T) {
}
