use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let counter = nums
            .iter() //
            .fold(HashMap::new(), |mut acc, &e| {
                acc.entry(e).and_modify(|v| *v += 1).or_insert(1);
                acc
            });

        let mut res = 0;
        for &a in counter.keys() {
            let b = k - a;
            res += match (counter.get(&a), counter.get(&b)) {
                (Some(&m), Some(&n)) if a < b => m.min(n),
                (Some(&m), Some(_)) if a == b => m / 2,
                _ => 0,
            }
        }
        res
    }
}
