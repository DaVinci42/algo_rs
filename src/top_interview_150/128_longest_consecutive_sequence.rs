use std::collections::{HashMap, HashSet};

struct UnionFind {
    id: HashMap<i32, i32>,
    size: HashMap<i32, usize>,
}

impl UnionFind {
    fn new(s: &HashSet<i32>) -> Self {
        Self {
            id: s.iter().map(|&n| (n, n)).collect(),
            size: s.iter().map(|&n| (n, 1)).collect(),
        }
    }

    fn find(&mut self, p: i32) -> i32 {
        let parent = self.id[&p];
        if parent != self.id[&parent] {
            let root = self.find(parent);
            self.id.insert(p, root);
        }
        self.id[&p]
    }

    fn union(&mut self, p: i32, q: i32) {
        let (p_root, q_root) = (self.find(p), self.find(q));
        if p_root == q_root {
            return;
        }

        let (p_size, q_size) = (self.size[&p_root], self.size[&q_root]);
        if p_size <= q_size {
            self.id.insert(p_root, q_root);
            self.size.insert(q_root, q_size + p_size);
        } else {
            self.id.insert(q_root, p_root);
            self.size.insert(p_root, p_size + q_size);
        }
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<_> = nums.iter().cloned().collect();
        let mut uf = UnionFind::new(&nums);
        for &n in nums.iter() {
            if nums.contains(&(n + 1)) {
                uf.union(n, n + 1);
            }
        }
        *uf.size.values().max().unwrap_or(&0) as i32
    }
}
