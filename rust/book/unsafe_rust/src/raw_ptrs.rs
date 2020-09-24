pub fn raw_pointer() {
    let mut num = 5;

    // create raw immutable raw pointer
    let r1 = &num as *const i32;
    // create mutable raw pointer
    let r2 = &mut num as *mut i32;

    // using raw pointers is only allowed if placed in an 'unsafe' block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
