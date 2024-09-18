use std::collections::{HashMap, HashSet};

struct UnionFind {
    id: HashMap<(usize, usize), (usize, usize)>,
    size: HashMap<(usize, usize), usize>,
}

impl UnionFind {
    fn new(row: usize, col: usize) -> Self {
        Self {
            id: (0..row)
                .flat_map(|r| (0..col).map(move |c| ((r, c), (r, c))))
                .collect(),
            size: (0..row)
                .flat_map(|r| (0..col).map(move |c| ((r, c), 1)))
                .collect(),
        }
    }

    fn find(&mut self, p: (usize, usize)) -> (usize, usize) {
        let parent = self.id[&p];
        if parent != self.id[&parent] {
            let root = self.find(parent);
            self.id.insert(p, root);
        }
        self.id[&p]
    }

    fn union(&mut self, p: (usize, usize), q: (usize, usize)) {
        let (p_root, q_root) = (self.find(p), self.find(q));
        if p_root == q_root {
            return;
        }

        let (p_size, q_size) = (self.size[&p_root], self.size[&q_root]);
        if p_size <= q_size {
            self.id.insert(p_root, q_root);
            self.size.insert(q_root, p_size + q_size);
        } else {
            self.id.insert(q_root, p_root);
            self.size.insert(p_root, p_size + q_size);
        }
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (row, col) = (board.len(), board[0].len());
        let mut uf = UnionFind::new(row, col);
        for r in 0..row {
            for c in 0..col {
                if board[r][c] != 'O' {
                    continue;
                }

                if c + 1 < col && board[r][c + 1] == 'O' {
                    uf.union((r, c), (r, c + 1));
                }

                if r + 1 < row && board[r + 1][c] == 'O' {
                    uf.union((r, c), (r + 1, c));
                }
            }
        }

        let mut invalid_set = HashSet::new();
        for c in 0..col {
            if board[0][c] == 'O' {
                invalid_set.insert(uf.find((0, c)));
            }
            if board[row - 1][c] == 'O' {
                invalid_set.insert(uf.find((row - 1, c)));
            }
        }

        for r in 0..row {
            if board[r][0] == 'O' {
                invalid_set.insert(uf.find((r, 0)));
            }
            if board[r][col - 1] == 'O' {
                invalid_set.insert(uf.find((r, col - 1)));
            }
        }

        for r in 0..row {
            for c in 0..col {
                if board[r][c] != 'O' || invalid_set.contains(&uf.find((r, c))) {
                    continue;
                }
                board[r][c] = 'X';
            }
        }
    }
}
