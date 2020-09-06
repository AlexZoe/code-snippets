// Classic enum type
enum IpAddrType {
    v4,
    v6
}

// struct using enum
struct IpAddrS {
    kind: IpAddrType,
    addr: String
}

// enum type with data attached
enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String)
}

// A more complex enum type
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(u8, u8, u8)
}

// enum type can also have member functions
impl Message {
    fn call(&self) {
        // do something
    }
}

fn main() {
    // Access enum type
    let four = IpAddrType::v4;
    let six = IpAddrType::v6;

    let home = IpAddrS {
        kind: four,
        addr: String::from("127.0.0.1")
    };

    let loopback = IpAddr::v6(String::from("::1"));
    let new_home = IpAddr::v4(127, 0, 0, 1);
}
