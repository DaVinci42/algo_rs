impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (-1, nums.len() as i32);
        while left + 1 < right {
            let mid = ((left + right) / 2) as usize;
            if nums[mid] < target {
                left = mid as i32;
            } else {
                right = mid as i32;
            }
        }
        right
    }
}
