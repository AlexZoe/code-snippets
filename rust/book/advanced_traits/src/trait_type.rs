use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

// add 'Meters' struct to 'Millimeters' struct
impl Add<Meters> for Millimeters {
    // return type
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// use default type (Millimeters)
impl Add for Millimeters {
    // return type
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + (other.0))
    }
}

pub fn use_trait_type() {
    let a = Meters(1);
    let b = Millimeters(1);

    let c = b + a;
    println!("result distance: {:?}", c);

    let one = Millimeters(1);
    let d = c + one;
    println!("result distance: {:?}", d);
}
