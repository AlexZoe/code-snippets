use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None
        }
    }
}

struct SingleList<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> SingleList<T> {
    fn new() -> Self {
        Self { first: None }
    }

    fn append(&mut self, data: T) {
        if self.first.is_some() {
            let mut item = self.first.clone();
            while let Some(current) = item {
                if current.borrow().next.is_none() {
                    current.borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(data))));
                    break;
                }
                item = current.borrow().next.clone();
            }
        } else {
            self.first = Some(Rc::new(RefCell::new(Node::new(data))));
        };
    }
}

impl<T: fmt::Display> fmt::Display for SingleList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut item = self.first.clone();
        while let Some(current) = item {
            write!(f, "{}", current.borrow().data)?;
            item = current.borrow().next.clone();
            if item.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}


fn main() {
    let mut my_list = SingleList::<u32>::new();
    my_list.append(3);
    my_list.append(2);
    my_list.append(8);
    println!("{}", my_list);
}
