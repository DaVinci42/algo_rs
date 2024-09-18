const NEIGHBOUR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

impl Solution {
    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
        grid[r][c] = '0';
        let (row, col) = (grid.len(), grid[0].len());
        NEIGHBOUR
            .iter()
            .map(|&(a, b)| (a + r as i32, b + c as i32))
            .filter(|&(a, b)| (0..row as i32).contains(&a) && (0..col as i32).contains(&b))
            .for_each(|(r, c)| {
                let (r, c) = (r as usize, c as usize);
                if grid[r][c] == '1' {
                    Self::dfs(grid, r, c);
                }
            });
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        let mut res = 0;
        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == '0' {
                    continue;
                }
                Self::dfs(&mut grid, r, c);
                res += 1;
            }
        }
        res
    }
}
