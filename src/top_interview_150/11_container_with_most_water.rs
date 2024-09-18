impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut res) = (0, height.len() - 1, 0);
        while left < right {
            res = res.max((right - left) as i32 * height[left].min(height[right]));
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        res
    }
}
