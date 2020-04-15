pub struct Stack {
    Items: Vec<i32>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            Items: Vec::new(),
        }
    }

    pub fn push(&mut self, data: i32) {
        self.Items.push(data);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.Items.pop()
    }
}

fn main() {
    let mut my_stack = Stack::new();
    my_stack.push(7);
    my_stack.push(2);
    let item = my_stack.pop();
    println!("item: {}", item.unwrap().to_string());
}
