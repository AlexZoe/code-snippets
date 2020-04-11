use std::fmt;

pub struct CoordinateSingleWrite {
    x: i32,
    y: i32,
}

impl CoordinateSingleWrite {

    pub fn new(x: i32, y: i32) -> CoordinateSingleWrite {
        CoordinateSingleWrite {
            x: x,
            y: y,
        }
    }
}

impl fmt::Display for CoordinateSingleWrite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

impl fmt::LowerHex for CoordinateSingleWrite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x: 0x{:x}, y: 0x{:x}}}", self.x, self.y)
    }
}

impl fmt::UpperHex for CoordinateSingleWrite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut width = 0;
        if f.width().is_some() {
            width = f.width().unwrap();
        }
        write!(f, "{{x: 0x{:0>width$X}, y: 0x{:0>width$X}}}", self.x, self.y, width = width)
    }
}

impl fmt::Binary for CoordinateSingleWrite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x: {:b}, y: {:b}}}", self.x, self.y)
    }
}


pub struct CoordinateMultiWrite {
    x: i32,
    y: i32,
}

impl CoordinateMultiWrite {

    pub fn new(x: i32, y: i32) -> CoordinateMultiWrite {
        CoordinateMultiWrite {
            x: x,
            y: y,
        }
    }
}

impl fmt::Display for CoordinateMultiWrite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, "\n\tx: {},", self.x)?;
        write!(f, "\n\ty: {}\n", self.y)?;
        write!(f, "}}")
    }
}

fn main () {
    let coord = CoordinateSingleWrite::new(3, 1);
    println!("Decimal Coord: {}", coord);

    let coord_lower_hex = CoordinateSingleWrite::new(13, 10);
    println!("Hex Coord: {:x}", coord_lower_hex);

    let coord_upper_hex = CoordinateSingleWrite::new(130, 10);
    println!("Hex Coord: {:8X}", coord_upper_hex);

    let coord_binary = CoordinateSingleWrite::new(13, 10);
    println!("Binary Coord: {:b}", coord_binary);

    let coord2 = CoordinateMultiWrite::new(5, 7);
    println!("Coord: {}", coord2);
    println!("x: {}, y: {}", coord2.x, coord2.y);
}
