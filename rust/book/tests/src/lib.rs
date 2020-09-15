#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // this test gets ignored and will not be run
    #[test]
    #[ignore]
    fn this_too() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Fail this one");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // generic assert; Rust also offers assert_eq, assert_ne, ...
        assert!(larger.can_hold(&smaller));
        // use custom error message which uses format
        assert!(larger.can_hold(&smaller), "my error msg");
    }

    #[test]
    // this test is expected to panic and should contain the following error msg
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
