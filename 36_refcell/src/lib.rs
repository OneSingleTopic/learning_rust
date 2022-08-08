pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    limit_max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, limit_max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            limit_max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.limit_max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            one_borrow.push(String::from(message));
            drop(one_borrow);
            let mut two_borrow = self.sent_messages.borrow_mut();
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 10);

        limit_tracker.set_value(9);

        assert_eq!(
            mock_messenger.sent_messages.borrow().len(),
            2,
            "The mock messenger should have sent 2 messages but sent {}",
            mock_messenger.sent_messages.borrow().len()
        );
    }
}
