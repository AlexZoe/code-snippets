use std::rc::Rc;
use self::List::{Cons, Nil};


enum List {
    Cons(i32, Rc<List>),
    Nil,
}


pub fn my_refcount() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // clone will not make a deepcopy, instead, it will increase the reference count to 'a'
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // print the current reference count of 'a'
    println!("count after creating c = {}", Rc::strong_count(&a));
}
