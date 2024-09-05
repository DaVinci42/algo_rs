impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        (0..32_usize) //
            .map(|i| {
                let (a, b, c) = ((a >> i) & 1, (b >> i) & 1, (c >> i) & 1);
                if c == 1 {
                    (a | b) ^ 1
                } else {
                    a + b
                }
            })
            .sum()
    }
}
