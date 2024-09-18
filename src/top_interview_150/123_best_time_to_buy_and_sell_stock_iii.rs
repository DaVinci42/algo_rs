impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut empty, mut buy1, mut sold1, mut buy2, mut sold2) = (
            0_i64,
            i32::MIN as i64,
            i32::MIN as i64,
            i32::MIN as i64,
            i32::MIN as i64,
        );
        for &p in prices.iter() {
            let p = p as i64;
            (empty, buy1, sold1, buy2, sold2) = (
                empty,
                buy1.max(empty - p),
                sold1.max(buy1 + p),
                buy2.max(sold1 - p),
                sold2.max(buy2 + p),
            );
        }

        empty.max(buy1).max(sold1).max(buy2).max(sold2) as i32
    }
}
