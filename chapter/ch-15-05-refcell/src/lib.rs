pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("Error! You are over your quota");
        } else if percentage_of_max > 0.9 {
            self.messenger.send("Warning! Your are used up over 90%");
        } else if percentage_of_max > 0.75 {
            self.messenger.send("Info! Your are used up over 75%");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    struct MockMessager {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> Self {
            Self {
                sent_message: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessager {
        fn send(&self, msg: &str) {
            self.sent_message.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn percent_test() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messager.sent_message.borrow().len(), 1);
    }
}
