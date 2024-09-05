use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let (mut left, mut right) = (0, costs.len() - 1);
        let (mut res, k, candidates) = (0, k as usize, candidates as usize);
        let (mut left_heap, mut right_heap) = (BinaryHeap::new(), BinaryHeap::new());
        for _ in 0..k {
            while left <= right && left_heap.len() < candidates {
                left_heap.push(Reverse(costs[left]));
                left += 1;
            }
            while left <= right && right_heap.len() < candidates {
                right_heap.push(Reverse(costs[right]));
                right -= 1;
            }

            let left = left_heap.peek().map_or(i32::MAX, |&r| r.0);
            let right = right_heap.peek().map_or(i32::MAX, |&r| r.0);
            if left <= right {
                left_heap.pop();
                res += left as i64;
            } else {
                right_heap.pop();
                res += right as i64;
            }
        }
        res
    }
}
