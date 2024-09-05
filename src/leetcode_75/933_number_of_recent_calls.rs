use std::collections::VecDeque;

struct RecentCounter {
    deque: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            deque: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.deque.push_back(t);
        while let Some(&p) = self.deque.front() {
            if p < t - 3000 {
                self.deque.pop_front();
            } else {
                break;
            }
        }
        self.deque.len() as i32
    }
}
