impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut cache = vec![vec![0; n]; m];
        for r in 0..m {
            for c in 0..n {
                cache[r][c] = match (r, c) {
                    (0, 0) => 1,
                    (0, _) => cache[0][c - 1],
                    (_, 0) => cache[r - 1][0],
                    _ => cache[r - 1][c] + cache[r][c - 1],
                }
            }
        }
        *cache.last().unwrap().last().unwrap()
    }
}
