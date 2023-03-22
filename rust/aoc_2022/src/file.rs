use std::fs::File;
use std::io::Read;

pub fn get_string_from_file(file: &str) -> String {
    let mut s = String::new();
    let _ = match File::open(file) {
        Ok(mut f) => f.read_to_string(&mut s),
        _ => Ok(0),
    };
    s
}
