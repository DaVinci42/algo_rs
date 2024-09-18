impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        (0..32) //
            .filter(|&i| (n >> i) & 1 != 0)
            .count() as i32
    }
}
