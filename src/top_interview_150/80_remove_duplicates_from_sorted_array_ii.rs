impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut pos = 2;
        for i in 2..nums.len() {
            if nums[pos - 2] == nums[i] {
                continue;
            }
            nums[pos] = nums[i];
            pos += 1;
        }
        pos.min(nums.len()) as i32
    }
}
