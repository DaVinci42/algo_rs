impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        fn reverse(nums: &mut Vec<i32>, mut left: usize, mut right: usize) {
            while left < right {
                (nums[left], nums[right]) = (nums[right], nums[left]);
                (left, right) = (left + 1, right - 1);
            }
        }
        let k = (k as usize) % nums.len();
        if k == 0 || nums.len() == 1 {
            return;
        }
        reverse(nums, 0, nums.len() - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, nums.len() - 1);
    }
}
