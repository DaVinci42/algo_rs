use rand::prelude::*;
use std::collections::HashMap;

struct RandomizedSet {
    nums: Vec<i32>,
    index_map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            nums: vec![],
            index_map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.index_map.contains_key(&val) {
            return false;
        }

        let i = self.nums.len();
        self.nums.push(val);
        self.index_map.insert(val, i);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&i) = self.index_map.get(&val) {
            let last = *self.nums.last().unwrap();
            let last_pos = self.index_map[&last];
            self.nums.swap(i, last_pos);
            self.index_map.insert(last, i);

            self.nums.pop();
            self.index_map.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.nums.len());
        self.nums[i]
    }
}
