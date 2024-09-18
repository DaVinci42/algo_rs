impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (row, col) = (matrix.len(), matrix[0].len());
        let (mut top, mut bottom) = (-1, row as i32);
        while top + 1 < bottom {
            let mid = (top + bottom) / 2;
            if matrix[mid as usize][0] <= target {
                top = mid;
            } else {
                bottom = mid;
            }
        }
        if top == -1 {
            return false;
        }
        let top = top as usize;
        let (mut left, mut right) = (-1, col as i32);
        while left + 1 < right {
            let mid = (left + right) / 2;
            if matrix[top][mid as usize] <= target {
                left = mid;
            } else {
                right = mid;
            }
        }
        left >= 0 && matrix[top][left as usize] == target
    }
}
