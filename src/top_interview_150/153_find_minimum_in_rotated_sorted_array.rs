impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (-1, nums.len() as i32);
        while left + 1 < right {
            let mid = (left + right) / 2;
            if nums[mid as usize] >= nums[0] {
                left = mid;
            } else {
                right = mid;
            }
        }
        let right = right as usize;
        if right == nums.len() {
            nums[0]
        } else {
            nums[right]
        }
    }
}
