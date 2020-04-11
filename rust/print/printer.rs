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
    println!("Coord: {}", coord);
    let coord2 = CoordinateMultiWrite::new(5, 7);
    println!("Coord: {}", coord2);
}
