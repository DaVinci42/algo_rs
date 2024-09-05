use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut res, mut left, mut right) = (0, 0, height.len() - 1);
        while left < right {
            res = res.max(
                (right - left) as i32
                    * cmp::min(
                        height[left], //
                        height[right],
                    ),
            );
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        res
    }
}
