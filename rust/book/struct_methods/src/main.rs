#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // member function
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // member function
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // associated function which does not take struct instance as first parameter
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side
        }
    }
}


// can have multiple impl blocks
impl Rectangle {
    fn print(&self) {
        println!("rectangle is: {:#?}", self);
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    rect1.print();

    // call associated function
    let new_square = Rectangle::square(3);
    println!("new square is {:?}", new_square);
}

