use std::cell::RefCell;
use std::rc::{Rc, Weak};
use rand::prelude::*;


struct Node<T> {
    val: T,
    parent: Option<Weak<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}


pub struct BTree<T> {
    size: usize,
    root: Option<Rc<RefCell<Node<T>>>>,
}

fn create_node<T>(val: T, parent: Option<Weak<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
    Some(Rc::new(RefCell::new(Node {
        val: val,
        parent: parent,
        left: None,
        right: None,
    })))
}

impl<T: std::cmp::PartialOrd> BTree<T> {
    fn new() -> Self {
        Self {
            root: None,
            size: 0
        }
    }

    fn size(&mut self) -> usize {
        self.size
    }

    fn add(&mut self, val: T) {
        if self.root.is_none() {
            self.root = create_node(val, None);
            self.size = 1;
        } else {
            let mut current_node = self.root.clone();
            while current_node.is_some() {
                if val < current_node.as_ref().unwrap().borrow().val {
                    if current_node.as_ref().unwrap().borrow().left.is_none() {
                        current_node.unwrap().borrow_mut().left =
                                create_node(val, Some(Rc::downgrade(&current_node.as_ref().unwrap().clone())));
                        break;
                    } else {
                        current_node = current_node.unwrap().borrow_mut().left.clone();
                    }
                } else if val > current_node.as_ref().unwrap().borrow().val {
                    if current_node.as_ref().unwrap().borrow().right.is_none() {
                        current_node.unwrap().borrow_mut().right =
                                create_node(val, Some(Rc::downgrade(&current_node.as_ref().unwrap().clone())));
                        break;
                    } else {
                        current_node = current_node.unwrap().borrow_mut().right.clone();
                    }
                } else {
                    current_node.as_ref().unwrap().borrow_mut().val = val;
                    return;
                }
            }
            self.size += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_tree() {
        let mut bt = BTree::<u32>::new();
        assert_eq!(0, bt.size());
    }

    #[test]
    fn add_item() {
        let mut bt = BTree::<u32>::new();
        bt.add(random());
        assert!(bt.size() > 0)
    }

    #[test]
    fn add_two_item() {
        let mut bt = BTree::<u32>::new();
        bt.add(random());
        bt.add(random());
        assert!(bt.size() > 1)
    }
}
