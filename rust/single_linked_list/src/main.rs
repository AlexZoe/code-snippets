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
    length: usize,
    first: Option<Rc<RefCell<Node<T>>>>,
}

impl<T:Copy> SingleList<T> {
    fn new() -> Self {
        Self {
            first: None,
            length: 0
        }
    }

    fn append(&mut self, data: T) {
        self.length += 1;
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

    fn prepend(&mut self, data: T) {
        self.length += 1;
        let prev_first = self.first.clone();
        self.first = Some(Rc::new(RefCell::new(Node::new(data))));
        self.first.as_ref().unwrap().borrow_mut().next = prev_first;
    }

    fn at(&self, idx: usize) -> Option<T> {
        if self.length > idx {
            let mut item = self.first.clone();
            for _ in 0..idx {
                item = item.unwrap().borrow().next.clone();
            }
            return Some(item.unwrap().borrow().data)
        } else {
            None
        }
    }

    fn delete(&mut self, idx: usize) {
        if self.length > idx {
            self.length -= 1;
            if idx == 0 {
                let item = self.first.as_ref().unwrap().borrow().next.clone();
                self.first = item;
            } else {
                let mut item = self.first.clone();
                let mut delete_candidate = self.first.as_ref().unwrap().borrow().next.clone();
                for _ in 0..idx-1 {
                    item = item.unwrap().borrow().next.clone();
                    delete_candidate = delete_candidate.unwrap().borrow().next.clone();
                }
                item.unwrap().borrow_mut().next = delete_candidate.unwrap().borrow().next.clone();
            }
        }
    }

    fn length(&self) -> &usize {
        &self.length
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

impl<T:Copy> IntoIterator for SingleList<T> {
    type Item = T;
    type IntoIter = SingleListIteratorState<T>;
    fn into_iter(self) -> SingleListIteratorState<T> {
        SingleListIteratorState{
            next: self.first
        }
    }
}

struct SingleListIteratorState<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T:Copy> Iterator for SingleListIteratorState<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.next.is_some() {
            let ret = Some(self.next.as_ref().unwrap().borrow().data);
            let tmp = self.next.as_ref().unwrap().borrow().next.clone();
            self.next = tmp;
            ret
        } else {
            None
        }
    }
}

fn main() {
    let mut my_list = SingleList::<u32>::new();
    my_list.append(3);
    my_list.append(2);
    my_list.delete(0);
    my_list.append(8);
    my_list.prepend(1);
    my_list.delete(1);
    my_list.append(7);
    println!("list has {} elements", my_list.length());
    println!("{}", my_list);
    println!("val at {}: {}", 2, my_list.at(2).unwrap());

    for i in my_list {
        println!("{}, ", i);
    }
}
