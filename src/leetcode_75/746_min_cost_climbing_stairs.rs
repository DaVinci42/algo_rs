use std::cmp;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; cost.len() + 1];
        (dp[0], dp[1]) = (0, 0);
        for i in 2..dp.len() {
            dp[i] = cmp::min(
                dp[i - 2] + cost[i - 2], //
                dp[i - 1] + cost[i - 1],
            );
        }
        *dp.last().unwrap()
    }
}
