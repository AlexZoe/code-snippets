use std;

/*
The rust compiler is pretty good, it will suggest to use the '#[repr(C)]'
directive as part of a warning to make sure that data fields are aligned
the same way as in C/C++.
See also:
https://doc.rust-lang.org/nomicon/other-reprs.html
*/
#[repr(C)]
pub struct Coordinate {
    x: i32,
    y: i32
}

// Use the Display trait to print the data fields with !println macros
impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

// The usual directive to prevent mangling the function name to allow linking with C/C++
#[no_mangle]
pub extern "C" fn share_struct_by_val(coord: Coordinate) {
    println!("{}", coord);
}

#[no_mangle]
pub extern "C" fn share_struct_by_ptr(coord: *const Coordinate) {
    // Decode struct pointer as array of size 1
    let coord = unsafe { std::slice::from_raw_parts(coord as *const Coordinate, 1) };
    /*
    Dereferencing with *coord won't work but treating the pointer as an array with size
    1 seems to work just fine. Valgrind also does not seem to have a problem with this.
    */
    println!("{}", coord[0]);
}
