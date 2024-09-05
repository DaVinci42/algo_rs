use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut occupied, mut empty) = (nums[0], 0);
        nums.iter() //
            .skip(1)
            .for_each(|n| {
                (occupied, empty) = (
                    cmp::max(occupied, empty + n), //
                    cmp::max(occupied, empty),
                )
            });
        occupied.max(empty)
    }
}
