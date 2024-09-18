use std::cmp;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        (0..row)
            .flat_map(|r| {
                (0..col) //
                    .map(move |c| (r, c))
            })
            .for_each(|(r, c)| {
                grid[r][c] += match (r, c) {
                    (0, 0) => 0,
                    (0, _) => grid[r][c - 1],
                    (_, 0) => grid[r - 1][c],
                    _ => cmp::min(
                        grid[r - 1][c], //
                        grid[r][c - 1],
                    ),
                }
            });
        grid[row - 1][col - 1]
    }
}
