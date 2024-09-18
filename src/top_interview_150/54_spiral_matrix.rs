const DIRECTION: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut res = Vec::with_capacity(row * col);
        let (mut r, mut c, mut dir) = (0_i32, 0_i32, 0);
        for _ in 0..row * col {
            res.push(matrix[r as usize][c as usize]);
            matrix[r as usize][c as usize] = i32::MIN;

            let (new_r, new_c) = (
                r + DIRECTION[dir].0, //
                c + DIRECTION[dir].1,
            );
            if 0 <= new_r
                && new_r < row as i32
                && 0 <= new_c
                && new_c < col as i32
                && matrix[new_r as usize][new_c as usize] != i32::MIN
            {
                (r, c) = (new_r, new_c);
            } else {
                match dir {
                    0 => (r, c, dir) = (r + 1, c, 1),
                    1 => (r, c, dir) = (r, c - 1, 2),
                    2 => (r, c, dir) = (r - 1, c, 3),
                    _ => (r, c, dir) = (r, c + 1, 0),
                }
            }
        }
        res
    }
}
