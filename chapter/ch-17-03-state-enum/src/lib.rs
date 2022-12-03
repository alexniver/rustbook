pub struct Post {
    state: State,
    text: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: State::Draft,
            text: String::from(""),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn request_review(&mut self) {
        match self.state {
            State::Draft => {
                self.state = State::RequestReview;
            }
            _ => {}
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            State::RequestReview => {
                self.state = State::Approve;
            }
            _ => {}
        }
    }

    pub fn content(&self) -> &str {
        match self.state {
            State::Approve => self.text.as_str(),
            _ => "",
        }
    }
}

pub enum State {
    Draft,
    RequestReview,
    Approve,
}
