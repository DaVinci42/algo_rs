impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut rob, mut skip) = (i32::MIN, 0);
        for &n in nums.iter() {
            (rob, skip) = (rob.max(skip + n), skip.max(rob));
        }
        rob.max(skip)
    }
}
