impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut dp = vec![[i32::MIN, i32::MIN]; k];
        for &p in prices.iter() {
            dp[0] = [
                dp[0][0].max(-p), //
                dp[0][1].max(dp[0][0] + p),
            ];
            for i in 1..k {
                dp[i] = [
                    dp[i][0].max(dp[i - 1][1] - p), //
                    dp[i][1].max(dp[i][0] + p),
                ];
            }
        }
        dp.iter().map(|v| v[1]).max().unwrap_or(0).max(0)
    }
}
