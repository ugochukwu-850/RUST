pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_two(a: i32) -> i32 {
    a + 2
    }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn greeting(name: String) -> String {
    format!("Hello {name}")
}
#[cfg(test)]
mod explore {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((2 * 2), 4);
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        //adding extra message args like in python
        let result = greeting("Carol".to_string());
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn greater_than_100() {
        Guess::new(0);
    }

    #[test]
    #[ignore]
    fn it_work2s() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
