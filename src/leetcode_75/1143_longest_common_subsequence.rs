use std::cmp;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (s1, s2) = (text1.as_bytes(), text2.as_bytes());
        let (r, c) = (s1.len(), s2.len());
        let mut dp = vec![vec![0; c]; r];
        for i in (0..r).rev() {
            for j in (0..c).rev() {
                dp[i][j] = if s1[i] == s2[j] {
                    1 + dp.get(i + 1).and_then(|row| row.get(j + 1)).unwrap_or(&0)
                } else {
                    cmp::max(
                        *dp.get(i).and_then(|row| row.get(j + 1)).unwrap_or(&0),
                        *dp.get(i + 1).and_then(|row| row.get(j)).unwrap_or(&0),
                    )
                }
            }
        }
        dp[0][0]
    }
}
