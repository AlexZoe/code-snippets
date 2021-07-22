use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {    
            val: val,
            next: None,
        }        
    }
}

pub struct Queue<T> {
    num_elems: usize,
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Queue<T>
    where T: Copy {
    pub fn new() -> Self {
        Queue {
            num_elems: 0,
            first: None,
            last: None,
        }
    }

    pub fn push(&mut self, val: T) {
        let new_item = Rc::new(RefCell::new(Node::new(val)));
        if self.num_elems > 0 {
            self.last.as_ref().unwrap().borrow_mut().next =
                Some(new_item.clone());
        } else {
            self.first =
                Some(new_item.clone());
        }
        self.last = Some(new_item);
        self.num_elems += 1;
    }

    pub fn pop(&mut self) {
        if self.num_elems > 0 {
            let new_first =
                self.first.as_ref().unwrap().borrow().next.clone();
            self.first = new_first;
            self.num_elems -= 1;
        }
    }

    pub fn front(&self) -> Option<T> {
        match &self.first {
            Some(item) => return Some(item.borrow().val),
            None => return None
        };
    }

    pub fn back(&self) -> Option<T> {
        match &self.last {
            Some(item) => return Some(item.borrow().val),
            None => return None
        };
    }

    pub fn size(&self) -> usize {
        self.num_elems
    }

    pub fn empty(&self) -> bool {
        match self.num_elems {
            0 => return false,
            _ => return true
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn newQeue() {
        let queue = Queue::<u32>::new();
    }

    #[test]
    fn pushItem() {
        let mut queue = Queue::<u32>::new();
        for i in 0..5 {
            queue.push(i);
            assert!(queue.size() == (i as usize + 1));
            assert!(queue.back().unwrap() == i);
        }
    }

    #[test]
    fn popItem() {
        let mut queue = Queue::<u32>::new();
        for i in 0..5 {
            queue.push(i);
            assert!(queue.size() == (i as usize + 1));
        }

        let mut item_front: u32 = 0;
        while queue.size() > 0 {
            assert!(queue.front().unwrap() == item_front);
            item_front += 1;
            queue.pop();
        }
    }
}
