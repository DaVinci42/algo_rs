use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut hold, mut empty) = (i32::MIN, 0);
        for &p in prices.iter() {
            (hold, empty) = (hold.max(empty - p), empty.max(hold + p))
        }
        cmp::max(hold, empty)
    }
}
