struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn my_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // d gets invalidated here since it is moved to e
    let e = d;
    println!("Print assigned smart pointer: {}.", e.data);
    // c gets invalidated here since it is moved to e
    // the data initially assigned to d is no longer reachable but does not get
    // dropped until d gets out of scope
    let e = c;
    println!("Print previous one: {}.", e.data);
}


pub fn manual_drop() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
