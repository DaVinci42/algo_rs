impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut pre_min, mut res) = (prices[0], 0);
        for &p in prices.iter() {
            res = res.max(p - pre_min);
            pre_min = pre_min.min(p);
        }
        res
    }
}
