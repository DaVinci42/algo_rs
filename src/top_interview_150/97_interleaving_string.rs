impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        // match with length i, j
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        for i in 0..s1.len() + 1 {
            for j in 0..s2.len() + 1 {
                dp[i][j] = match (i, j) {
                    (0, 0) => true,
                    (0, _) => dp[0][j - 1] && s2[j - 1] == s3[i + j - 1],
                    (_, 0) => dp[i - 1][0] && s1[i - 1] == s3[i + j - 1],
                    _ => {
                        dp[i][j - 1] && s2[j - 1] == s3[i + j - 1]
                            || dp[i - 1][j] && s1[i - 1] == s3[i + j - 1]
                    }
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
}
