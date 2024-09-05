use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let s1: HashSet<_> = nums1.into_iter().collect();
        let s2: HashSet<_> = nums2.into_iter().collect();
        vec![
            s1.difference(&s2).cloned().collect(),
            s2.difference(&s1).cloned().collect(),
        ]
    }
}
