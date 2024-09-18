use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            if matches!(map.get(&n), Some(j) if i - j <= k) {
                return true;
            }
            map.insert(n, i);
        }
        false
    }
}
