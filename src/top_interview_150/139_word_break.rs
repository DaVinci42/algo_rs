use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut deque = VecDeque::from([0]);
        let mut visited = HashSet::from([0]);
        let target = s.as_str().len();
        while let Some(start) = deque.pop_front() {
            if start == target {
                return true;
            }
            for w in word_dict.iter() {
                let word_len = w.as_str().len();
                let ending = start + word_len - 1;
                if ending >= target || &s.as_str()[start..=ending] != w.as_str() {
                    continue;
                }
                if visited.contains(&(ending + 1)) || ending + 1 > target {
                    continue;
                }
                visited.insert(ending + 1);
                deque.push_back(ending + 1);
            }
        }
        false
    }
}
