use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Stack<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl <T> Stack<T>
where T: Copy {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let mut old_root: Option<Rc<RefCell<Node<T>>>> = None;
        if self.root.is_some() {
            old_root = self.root.clone();
        };
        self.root = Some(Rc::new(RefCell::new(Node {
            next: old_root,
            val: value,
        })));
        self.size += 1;
    }

    pub fn top(&self) -> Option<T> {
        match &self.root {
            None => return None,
            Some(root) => return Some(root.borrow().val),
        };
    }

    pub fn pop(&mut self) {
        if let Some(old_root) = self.root.clone() {
            self.root = old_root.borrow().next.clone();
            self.size -= 1;
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[test]
fn newStack() {
    let stack = Stack::<u32>::new();
}

#[test]
fn pushValues() {
    let mut stack = Stack::<u32>::new();
    for i in 1..5 {
        stack.push(i);
    }
}

#[test]
fn popValuesAfterPush() {
    let mut stack = Stack::<u32>::new();
    for i in 1..5 {
        stack.push(i);
    }

    while stack.size() > 0 {
        println!("val: {}", stack.top().unwrap());
        stack.pop();
    }
}
