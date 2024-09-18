impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farest = 0;
        for (i, &n) in nums.iter().enumerate() {
            if i > farest {
                return false;
            }
            farest = farest.max(i + n as usize);
            if farest >= nums.len() - 1 {
                return true;
            }
        }
        unreachable!()
    }
}
