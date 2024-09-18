impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        (0..32_usize)
            .rev()
            .take_while(|i| (left >> i) & 1 == (right >> i) & 1)
            .fold(0, |acc, e| {
                if (left >> e) & 1 == 1 {
                    acc | 1 << e
                } else {
                    acc
                }
            })
    }
}