const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut dp = (0, 1, 0, 0);
        for _ in 0..n as usize {
            let (empty, full, top, bottom) = dp;
            dp = (
                full,
                (full + empty + top + bottom) % MOD,
                (empty + bottom) % MOD,
                (empty + top) % MOD,
            );
        }
        dp.1 as i32
    }
}
