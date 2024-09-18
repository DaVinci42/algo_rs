use std::collections::{HashSet, VecDeque};

const NEIGHBOUR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let (row, col) = (board.len(), board[0].len());
        let mut deque: VecDeque<_> = (0..row)
            .flat_map(|r| {
                (0..col) //
                    .map(move |c| (r, c))
            })
            .filter_map(|(r, c)| {
                if board[r][c] == word[0] {
                    Some((r, c, HashSet::from([(r, c)])))
                } else {
                    None
                }
            })
            .collect();
        while let Some((r, c, visited)) = deque.pop_front() {
            if visited.len() == word.len() {
                return true;
            }
            let next = word[visited.len()];
            NEIGHBOUR
                .iter() //
                .map(|&(m, n)| (m + r as i32, n + c as i32))
                .filter(|&(r, c)| 0 <= r && r < row as i32 && 0 <= c && c < col as i32)
                .for_each(|(r, c)| {
                    let (r, c) = (r as usize, c as usize);
                    if board[r][c] != next || visited.contains(&(r, c)) {
                        return;
                    }

                    let mut visited = visited.clone();
                    visited.insert((r, c));
                    deque.push_back((r, c, visited))
                });
        }
        false
    }
}
