impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in 1..triangle.len() {
            for j in 0..=i {
                triangle[i][j] += match j {
                    0 => triangle[i - 1][0],
                    a if a == i => triangle[i - 1][i - 1],
                    _ => triangle[i - 1][j - 1].min(triangle[i - 1][j]),
                }
            }
        }
        *triangle.last().unwrap().iter().min().unwrap()
    }
}
