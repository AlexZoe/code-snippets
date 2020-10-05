use std::ops;
use std::fmt;

struct Coord2D {
    x: i32,
    y: i32,
}

impl std::cmp::PartialEq for Coord2D {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        }
    }
}

impl ops::Add for Coord2D {
    type Output = Coord2D;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y }
    }
}

impl fmt::Display for Coord2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn main() {
    let first = Coord2D {x: 3, y:2};
    let second = Coord2D {x: 2, y:3};
    println!("coordinates are equal: {}", first == second);
    println!("coordinates are not equal: {}", first != first);

    println!("coordinate sum is: {}", first + second);
}

