impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
        // edit with length at i, j
        let mut dp = vec![vec![i32::MAX; s2.len() + 1]; s1.len() + 1];
        for i in 0..s1.len() + 1 {
            for j in 0..s2.len() + 1 {
                dp[i][j] = match (i, j) {
                    (0, 0) => 0,
                    (0, _) => j as i32,
                    (_, 0) => i as i32,
                    _ if s1[i - 1] == s2[j - 1] => dp[i - 1][j - 1],
                    _ => 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]),
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
}
