use std::collections::HashSet;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn backtrack(i: usize, board: &mut Vec<Vec<i32>>, res: &mut i32) {
            let n = board.len();
            if i == n {
                *res += 1;
            } else {
                let mut cols = HashSet::new();
                let mut main_diagonal = HashSet::new();
                let mut sub_diagon = HashSet::new();
                for r in 0..n {
                    for c in 0..n {
                        if board[r][c] == 0 {
                            continue;
                        }
                        cols.insert(c);
                        main_diagonal.insert(r + c);
                        sub_diagon.insert(r as i32 - c as i32);
                    }
                }

                let options: Vec<_> = (0..n)
                    .filter(|&c| {
                        !cols.contains(&c)
                            && !main_diagonal.contains(&(i + c))
                            && !sub_diagon.contains(&(i as i32 - c as i32))
                    })
                    .collect();
                for &c in options.iter() {
                    board[i][c] = 1;
                    backtrack(i + 1, board, res);
                    board[i][c] = 0;
                }
            }
        }

        let mut res = 0;
        let n = n as usize;
        let mut board = vec![vec![0; n]; n];
        backtrack(0, &mut board, &mut res);
        res
    }
}
