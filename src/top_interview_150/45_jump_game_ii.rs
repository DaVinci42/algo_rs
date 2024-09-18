impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let target = nums.len() - 1;
        let mut dp = vec![0; nums.len()];
        for (i, &n) in nums.iter().enumerate() {
            let n = n as usize;
            for j in (i + 1)..=(i + n) {
                if j >= nums.len() {
                    break;
                } else if j == target {
                    return dp[i] + 1;
                } else if dp[j] != 0 {
                    continue;
                }
                dp[j] = dp[i] + 1
            }
        }
        *dp.last().unwrap()
    }
}
