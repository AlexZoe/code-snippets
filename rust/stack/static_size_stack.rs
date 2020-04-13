const STACK_SIZE: usize = 10;

#[derive(Copy, Clone)]
pub enum Item {
    ElementContainer { element: i32},
    Null,
}

impl Item {
    pub fn new() -> Item {
        Item::Null
    }
}


pub struct MyStack {
    item_list: [Item; STACK_SIZE],
    size: usize,
}


impl MyStack {
    pub fn push (&mut self, data: i32) {
        self.item_list[self.size] = Item::ElementContainer { element: data };
        self.size += 1;
    }

    pub fn pop (&mut self) -> Option<i32> {
        let mut ret: Option<i32> = None;
        if self.size > 0 {
            match self.item_list[self.size - 1] {
                Item::Null => ret = None,
                Item::ElementContainer {element} => ret = Some(element),
            };
            self.item_list[self.size - 1] = Item::Null;
            self.size -= 1;
        }
        ret
    }

    pub fn new () -> MyStack {
        MyStack {
            item_list: [Item::new(); STACK_SIZE],
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn at(&self, idx: usize) -> Option<i32> {
        match self.item_list[idx] {
            Item::Null => None,
            Item::ElementContainer {element} => Some(element),
        }
    }
}


fn main () {
    let mut stack = MyStack::new();
    stack.push(2);
    stack.push(7);
    stack.push(3);
    println!("{}", stack.size());
    println!("{}", stack.at(1).unwrap().to_string());
    println!("{}", stack.pop().unwrap().to_string());
}
