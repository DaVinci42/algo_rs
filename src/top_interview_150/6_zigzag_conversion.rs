impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut grid = vec![String::new(); num_rows];
        let (mut i, mut dir) = (0, 1_i32);
        for c in s.chars() {
            grid[i].push(c);
            if i == num_rows - 1 {
                dir = -1;
            } else if i == 0 {
                dir = 1;
            }
            i = (i as i32 + dir) as usize;
        }
        grid.concat()
    }
}
