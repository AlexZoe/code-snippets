// implement debug trait to print struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect is {:?}", rectangle); // pring
    println!("rect is {:#?}", rectangle); // pretty pring
}
