impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (row, col) = (matrix.len(), matrix[0].len());
        for c in 0..col {
            for r in 0..row / 2 {
                (matrix[r][c], matrix[row - 1 - r][c]) = (matrix[row - 1 - r][c], matrix[r][c]);
            }
        }
        for r in 0..row {
            for c in 0..r {
                (matrix[r][c], matrix[c][r]) = (matrix[c][r], matrix[r][c]);
            }
        }
    }
}
