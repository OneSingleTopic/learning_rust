pub mod shapes;

pub struct Guess {
    pub number: u32,
}

impl Guess {
    pub fn new(number: u32) -> Guess {
        if number < 1 {
            panic!("Number should be greater than 0, got {}", number);
        } else if number > 100 {
            panic!("Number should be lower than 101, got {}", number);
        }
        Guess { number }
    }
}

pub fn add_two(number_0: u32) -> u32 {
    number_0 + 2
}

pub fn greeting(username: &str) -> String {
    format!("Hello {}", username)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn rectangle_can_hold() {
        let rectangle_0 = shapes::rectangle::Rectangle {
            width: 100,
            height: 100,
        };
        let rectangle_1 = shapes::rectangle::Rectangle {
            width: 40,
            height: 70,
        };

        assert!(rectangle_0.can_hold(&rectangle_1));
    }

    #[test]
    #[ignore]
    fn rectangle_cannot_hold() {
        let rectangle_0 = shapes::rectangle::Rectangle {
            width: 100,
            height: 100,
        };
        let rectangle_1 = shapes::rectangle::Rectangle {
            width: 120,
            height: 70,
        };

        assert!(!rectangle_0.can_hold(&rectangle_1));
    }

    #[test]
    fn assert_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn contiains_username() {
        let toto = "Toto";
        assert!(
            greeting(toto).contains(toto),
            "The greeting message should contain {} but was : {}",
            toto,
            greeting(toto)
        );
    }

    #[test]
    #[should_panic(expected = "Number should be greater than 0")]
    fn test_input_guess() {
        Guess::new(0);
    }
}
