use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut visited = HashSet::from([(0, 0)]);
        let mut heap = BinaryHeap::from([Reverse((nums1[0] + nums2[0], 0, 0))]);
        let mut res = Vec::with_capacity(k as usize);
        for _ in 0..k {
            let Reverse((_, i, j)) = heap.pop().unwrap();
            res.push(vec![nums1[i], nums2[j]]);

            if i + 1 < nums1.len() && !visited.contains(&(i + 1, j)) {
                heap.push(Reverse((nums1[i + 1] + nums2[j], i + 1, j)));
                visited.insert((i + 1, j));
            }
            if j + 1 < nums2.len() && !visited.contains(&(i, j + 1)) {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
                visited.insert((i, j + 1));
            }
        }
        res
    }
}
