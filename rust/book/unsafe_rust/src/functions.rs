use std::slice;

// the whole fucntion body is considered 'unsafe'
unsafe fn dangerous_function() {
}


fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // raw pointer to slice start address
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // create two mutable slices from original slice
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}


pub fn call_unsafe() {
    // call unsafe function
    unsafe {
        dangerous_function();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    // Call C function (all external function calls have to be wrapped in an 'unsafe' block)
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


extern "C" {
    fn abs(input: i32) -> i32;
}
