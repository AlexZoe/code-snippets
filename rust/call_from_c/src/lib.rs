#[no_mangle]
pub extern "C" fn print_string_with_string(string: *mut u8, size: i32) {
    /*
    This won't work since String seems to take ownership of data pointed by 'string' and
    subsequently tries to free the data. If 'string' is stack allocated this will lead to
    double free. For heap allocated data, the C part can skip freeing the memory to prevent
    the program from exiting with an error.
    In either case, Rust seems not be able to successfully free the memory since
    allocation-free part won't match.
    */
    let r_string = unsafe { String::from_raw_parts(string, size as usize, size as usize) };

    println!("{}", r_string);
}

#[no_mangle]
pub extern "C" fn print_string_with_u8(string: *const u8, size: i32) {
    let result = std::panic::catch_unwind( || {
        let r_string = unsafe { std::slice::from_raw_parts(string, size as usize) };
        println!("{}", std::str::from_utf8(r_string).unwrap());
    });
    if result.is_err() {
        eprintln!("Panick from rust");
    }
}

#[no_mangle]
pub extern "C" fn print_float_array(raw_float: *const f32, size: i32) {
    let result = std::panic::catch_unwind( || {
        let float = unsafe { std::slice::from_raw_parts(raw_float, size as usize) };
        println!("Print float array:");
        for i in 0..size {
            println!("{}", float[i as usize]);
        }
    });
    if result.is_err() {
        eprintln!("Panick from rust");
    }
}

#[no_mangle]
pub extern "C" fn print_float(float: f32) {
    println!("Print single float:");
    println!("{}", float);
}
