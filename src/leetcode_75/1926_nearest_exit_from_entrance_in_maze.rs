use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (row_size, col_size, mut step) = (maze.len(), maze[0].len(), 0);
        let (init_r, init_c) = (entrance[0] as usize, entrance[1] as usize);
        maze[init_r][init_c] = '+';
        let mut deque = VecDeque::from([(init_r, init_c)]);
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let (r, c) = deque.pop_front().unwrap();
                if (r == 0 || r == row_size - 1 || c == 0 || c == col_size - 1)
                    && (r, c) != (init_r, init_c)
                {
                    return step;
                }

                directions
                    .iter()
                    .map(|&(a, b)| (r as i32 + a, c as i32 + b))
                    .for_each(|(a, b)| {
                        if !(0..(row_size as i32)).contains(&a)
                            || !(0..(col_size as i32)).contains(&b)
                        {
                            return;
                        }
                        let (a, b) = (a as usize, b as usize);
                        if maze[a][b] == '.' {
                            deque.push_back((a, b));
                            maze[a][b] = '+';
                        }
                    });
            }
            step += 1;
        }
        -1
    }
}
