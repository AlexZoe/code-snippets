trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn choose_explicit_call() {
    println!("A baby dog is called a {}", Dog::baby_name());
    // the following does not work since the function of the trait is an associated
    // function which does not have a 'self' to infer the type
//    println!("A baby dog is called a {}", Animal::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
