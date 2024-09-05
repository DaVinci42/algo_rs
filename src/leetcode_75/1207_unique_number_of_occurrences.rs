use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let map: HashMap<_, usize> = arr
            .iter() //
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(e).or_default() += 1;
                acc
            });
        map.values().collect::<HashSet<_>>().len() == map.len()
    }
}
