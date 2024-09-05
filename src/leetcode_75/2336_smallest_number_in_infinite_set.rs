use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct SmallestInfiniteSet {
    heap: BinaryHeap<Reverse<i32>>,
    cur: HashSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            heap: (1..=1000) //
                .map(Reverse)
                .collect(),
            cur: (1..=1000).collect(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        let res = self.heap.pop().unwrap().0;
        self.cur.remove(&res);
        res
    }

    fn add_back(&mut self, num: i32) {
        if self.cur.contains(&num) {
            return;
        }

        self.heap.push(Reverse(num));
        self.cur.insert(num);
    }
}
