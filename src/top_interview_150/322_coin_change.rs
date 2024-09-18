use std::cmp;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;
        for i in 0..=amount {
            let cur_change = dp[i];
            if cur_change == i32::MAX {
                continue;
            }
            for &c in coins.iter() {
                let next_amount: usize = i + c as usize;
                if next_amount > amount {
                    continue;
                }
                dp[next_amount] = cmp::min(dp[next_amount], cur_change + 1);
            }
        }
        if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }
}
