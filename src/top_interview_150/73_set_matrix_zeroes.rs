use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut zero_rows, mut zero_cols) = (HashSet::new(), HashSet::new());
        let (row, col) = (matrix.len(), matrix[0].len());
        for r in 0..row {
            for c in 0..col {
                if matrix[r][c] != 0 {
                    continue;
                }
                zero_rows.insert(r);
                zero_cols.insert(c);
            }
        }

        for &r in zero_rows.iter() {
            for c in 0..col {
                matrix[r][c] = 0;
            }
        }
        for &c in zero_cols.iter() {
            for r in 0..row {
                matrix[r][c] = 0;
            }
        }
    }
}
