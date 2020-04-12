use std::fmt;

pub struct Address {
    street: &'static str,
    number: i32,
}

impl Address {

    pub fn new(street: &'static str, number: i32) -> Address {
        Address {
            street: street,
            number: number,
        }
    }

}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{street: {}, number: {}}}", self.street, self.number)
    }
}


fn main () {
    let address = Address::new("my_street", 12);
    println!("Address: {}", address);
}
