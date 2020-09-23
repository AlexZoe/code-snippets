pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

// state is encoded by returning a different struct type which is the new state
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}


// saves state internally; takes any object that implements the 'State' trait
pub struct PostWithState {
    state: Option<Box<dyn State>>,
    content: String,
}

impl PostWithState {
    pub fn new() -> PostWithState {
        PostWithState {
            state: Some(Box::new(DraftWithState {})),
            content: String::new(),
        }
    }
}

trait State {}

struct DraftWithState {}

// all structs have to implement 'State'
impl State for DraftWithState {}
