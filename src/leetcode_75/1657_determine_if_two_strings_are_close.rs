use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let counter = |s: &str| -> HashMap<char, usize> {
            s.chars().fold(HashMap::new(), |mut acc, e| {
                *acc.entry(e).or_default() += 1;
                acc
            })
        };

        let (map1, map2) = (counter(&word1), counter(&word2));
        if map1.keys().collect::<HashSet<_>>() != map2.keys().collect::<HashSet<_>>() {
            return false;
        }

        let mut v1: Vec<_> = map1.values().collect();
        let mut v2: Vec<_> = map2.values().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        v1 == v2
    }
}
