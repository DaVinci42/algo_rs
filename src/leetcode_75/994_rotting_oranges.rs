use std::collections::VecDeque;

const DIRECTION: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (row_size, col_size) = (grid.len(), grid[0].len());
        let mut fresh_count = 0;
        let mut deque = VecDeque::new();
        for r in 0..row_size {
            for c in 0..col_size {
                match grid[r][c] {
                    1 => fresh_count += 1,
                    2 => deque.push_back((r, c)),
                    _ => {}
                }
            }
        }
        let mut cur_time = 0;
        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let (r, c) = deque.pop_front().unwrap();
                let (r, c) = (r as i32, c as i32);
                DIRECTION
                    .iter() //
                    .map(|&(m, n)| (r + m, c + n))
                    .filter(|&(r, c)| {
                        0 <= r && r < row_size as i32 && 0 <= c && c < col_size as i32
                    })
                    .for_each(|(r, c)| {
                        let (r, c) = (r as usize, c as usize);
                        if grid[r][c] != 1 {
                            return;
                        }
                        deque.push_back((r, c));
                        fresh_count -= 1;
                        grid[r][c] = 2;
                    });
            }
            if !deque.is_empty() {
                cur_time += 1;
            }
        }
        if fresh_count == 0 {
            cur_time
        } else {
            -1
        }
    }
}
