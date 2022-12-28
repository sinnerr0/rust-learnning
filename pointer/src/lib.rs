pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("경고: 최댓값의 75%를 사용했습니다.");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("긴급 경고: 최댓값의 90%를 사용했습니다.");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("에러: 최댓값을 초과했습니다.");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.send_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}
