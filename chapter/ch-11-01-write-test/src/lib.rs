#[derive(Debug)]
struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    pub fn can_hold(&self, other: Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Self {
        // if value < 1 {
        if value < 1 || value > 100 {
            panic!("value should between 1-100");
        }

        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use crate::add_two;

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rect {
            width: 10.,
            height: 5.,
        };
        let smaller = Rect {
            width: 9.,
            height: 4.,
        };

        assert!(larger.can_hold(smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rect {
            width: 10.,
            height: 5.,
        };
        let smaller = Rect {
            width: 11.,
            height: 4.,
        };

        assert!(!smaller.can_hold(larger));
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Tom");
        assert!(
            result.contains("Tom"),
            "Greeting did not contain name 'Tom', value was: {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn guess_greater_than_100() {
        let result = Guess::new(101);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2!=4?"))
        }
    }
}
