impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for (i, &n) in nums.iter().enumerate() {
            for j in i + 1..nums.len() {
                let m = nums[j];
                if m <= n {
                    continue;
                }
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
        *dp.iter().max().unwrap_or(&0)
    }
}
