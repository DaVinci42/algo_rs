use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for &n in nums.iter() {
            heap.push(Reverse(n));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.peek().unwrap().0
    }
}
