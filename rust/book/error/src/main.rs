use std::fs::File;
use std::io;
use std::io::Read;

// function returning String or Error
fn read_username_from_file() -> Result<String, io::Error> {
    // the '?' operator will assign 'Ok' value to f or return Error from function (same as 'return Err(e)'
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // return type of open is 'Result'
    let f = File::open("hello.txt");

    // Result can be used to recover from error
    let f = match f {
        Ok(file) => file,
        // panic is a non-recoverable error
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

