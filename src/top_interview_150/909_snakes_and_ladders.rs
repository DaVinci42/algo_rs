use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
};

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut mapping = HashMap::new();
        let (mut r, mut c, mut val) = (n - 1, 0_i32, 1);
        loop {
            for _ in 0..n {
                mapping.insert(val, board[r][c as usize]);
                val += 1;
                c += 1;
            }
            if r == 0 {
                break;
            }
            (r, c) = (r - 1, c - 1);
            for _ in 0..n {
                mapping.insert(val, board[r][c as usize]);
                val += 1;
                c -= 1;
            }
            if r == 0 {
                break;
            }
            (r, c) = (r - 1, 0);
        }

        let mut deque = VecDeque::from([1]);
        let mut visited = HashSet::from([1]);
        let (mut step, target) = (0, (n * n) as i32);
        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let v = deque.pop_front().unwrap();
                if v == target {
                    return step;
                }
                for next in (v + 1)..=cmp::min(v + 6, target) {
                    let next = match mapping[&next] {
                        -1 => next,
                        a => a,
                    };
                    if visited.contains(&next) {
                        continue;
                    }
                    deque.push_back(next);
                    visited.insert(next);
                }
            }
            step += 1;
        }
        -1
    }
}
