impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; col]; row];

        for r in 0..row {
            for c in 0..col {
                if obstacle_grid[r][c] == 1 {
                    dp[r][c] = 0;
                    continue;
                }
                dp[r][c] = match (r, c) {
                    (0, 0) => 1,
                    (0, _) => dp[r][c - 1],
                    (_, 0) => dp[r - 1][c],
                    _ => dp[r][c - 1] + dp[r - 1][c],
                }
            }
        }
        dp[row - 1][col - 1]
    }
}