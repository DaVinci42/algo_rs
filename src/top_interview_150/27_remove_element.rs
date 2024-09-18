impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut pos = 0;
        for i in 0..nums.len() {
            if nums[i] == val {
                continue;
            }
            nums[pos] = nums[i];
            pos += 1;
        }
        pos as i32
    }
}
