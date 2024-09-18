impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        (0..32)
            .filter(|&i| (x >> i) & 1 != 0)
            .fold(0, |acc, e| acc | 1 << (31 - e))
    }
}
