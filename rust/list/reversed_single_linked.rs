pub enum MyList {
    Item(i32, Box<MyList>),
    Null,
}


impl MyList {
    pub fn new() -> MyList {
        MyList::Null
    }

    pub fn prepend(self, elem: i32) -> MyList {
        MyList::Item(elem, Box::new(self))
    }

    pub fn length(&self) -> u32 {
        match *self {
            MyList::Item(_, ref tail) => 1 + tail.length(),
            MyList::Null => 0
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            MyList::Item(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            MyList::Null => {
                format!("Null")
            },
        }
    }
}


fn main () {
    let mut list = MyList::new();
    list = list.prepend(2);
    list = list.prepend(7);
    list = list.prepend(3);
    println!("List has {} elements", list.length());
    println!("List: {}", list.stringify());
}
