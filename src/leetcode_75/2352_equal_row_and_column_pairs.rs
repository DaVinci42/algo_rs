use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let map = grid
            .iter() //
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(e).or_insert(0) += 1;
                acc
            });

        (0..grid.len())
            .map(|c| {
                let col = (0..grid.len()) //
                    .map(|r| grid[r][c])
                    .collect::<Vec<_>>();
                map.get(&col).unwrap_or(&0)
            })
            .sum()
    }
}
