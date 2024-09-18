use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let (begin, end) = (begin_word.as_bytes(), end_word.as_bytes());
        let bank: HashSet<_> = word_list.iter().map(|s| s.as_bytes()).collect();
        let mut deque = VecDeque::from([begin]);
        let mut step = 1;
        let mut visited = HashSet::from([begin]);
        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let cur = deque.pop_front().unwrap();
                if cur == end {
                    return step;
                }
                for &w in bank.iter() {
                    if (0..w.len()).filter(|&i| cur[i] != w[i]).count() != 1 || visited.contains(w)
                    {
                        continue;
                    }
                    deque.push_back(w);
                    visited.insert(w);
                }
            }
            step += 1;
        }
        0
    }
}
