use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if let Some(Reverse(v)) = self.right.pop() {
            self.left.push(v);
        }
        self.left.push(num);
        while self.left.len() - self.right.len() > 1 {
            let n = self.left.pop().unwrap();
            self.right.push(Reverse(n));
        }
    }

    fn find_median(&self) -> f64 {
        if (self.left.len() + self.right.len()) % 2 == 1 {
            *self.left.peek().unwrap() as f64
        } else {
            let mut res = 0;
            if let Some(&v) = self.left.peek() {
                res += v;
            }
            if let Some(&Reverse(v)) = self.right.peek() {
                res += v;
            }
            res as f64 / 2.0
        }
    }
}
