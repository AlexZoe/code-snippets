pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            items: Vec::<T>::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        self.items.push(data);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

fn main() {
    let mut my_stack = Stack::new();
    my_stack.push("7");
    my_stack.push("2");
    let item = my_stack.pop();
    println!("item: {}", item.unwrap().to_string());
    println!("item: {}", my_stack.pop().unwrap().to_string());
}
