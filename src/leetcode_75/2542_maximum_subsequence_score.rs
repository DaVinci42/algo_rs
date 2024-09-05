use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut nums: Vec<_> = (0..nums2.len())
            .map(|i| (nums2[i] as i64, nums1[i] as i64))
            .collect();
        nums.sort_unstable();
        let mut heap = BinaryHeap::new();
        let (mut res, mut cur_sum, k) = (0_i64, 0_i64, k as usize);
        for i in (0..nums.len()).rev() {
            let (n2, n1) = nums[i];
            heap.push(Reverse(n1));
            cur_sum += n1;
            if heap.len() < k {
                continue;
            }
            res = res.max(cur_sum * n2);
            cur_sum -= heap.pop().unwrap().0;
        }
        res
    }
}
