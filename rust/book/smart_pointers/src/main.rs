mod deref;
mod drop;
mod ref_count;
mod ref_cell;
mod weak_ref;

fn main() {
    deref::own_box_type();
    deref::my_deref_call();
    drop::my_drop();
    drop::manual_drop();
    ref_count::my_refcount();
    ref_cell::mutable_ref_count();
    weak_ref::weak_ref_cycle();
}


fn use_box() {
    // box is allocated on stack which holds a i32 value allocated on heap
    let b = Box::new(5);
    println!("b = {}", b);
}


fn use_reference() {
    let x = 5;
    // y holds a reference to x
    let y = &x;

    assert_eq!(5, x);
    // y has to be dereferenced to access the value of x
    assert_eq!(5, *y);
}


fn with_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
