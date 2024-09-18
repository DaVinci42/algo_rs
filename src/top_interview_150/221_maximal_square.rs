use std::cmp;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; col]; row];
        let mut res = 0;

        for i in 0..row {
            for j in 0..col {
                dp[i][j] = match (i, j) {
                    _ if matrix[i][j] == '0' => 0,
                    (_, 0) => 1,
                    (0, _) => 1,
                    _ => {
                        let (left, top, top_left) = (
                            dp[i][j - 1], //
                            dp[i - 1][j],
                            dp[i - 1][j - 1],
                        );
                        cmp::min(
                            1 + left.min(top_left), //
                            1 + top.min(top_left),
                        )
                    }
                };
                res = res.max(dp[i][j])
            }
        }
        res * res
    }
}
