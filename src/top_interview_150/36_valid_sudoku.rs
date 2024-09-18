use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        fn row(board: &Vec<Vec<char>>, r: usize) -> bool {
            let mut cur = HashSet::new();
            board[r]
                .iter() //
                .filter(|&c| *c != '.')
                .all(|c| cur.insert(c))
        }

        fn col(board: &Vec<Vec<char>>, c: usize) -> bool {
            let mut cur = HashSet::new();
            (0..9)
                .map(|r| board[r][c])
                .filter(|&c| c != '.')
                .all(|c| cur.insert(c))
        }

        fn subbox(board: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
            let mut cur = HashSet::new();
            (0..3)
                .flat_map(|i| {
                    (0..3) //
                        .map(move |j| &board[r + i][c + j])
                })
                .filter(|&c| *c != '.')
                .all(|c| cur.insert(c))
        }

        [0, 3, 6]
            .iter()
            .flat_map(|&r| {
                [0, 3, 6]
                    .iter() //
                    .map(move |&c| (r, c))
            })
            .all(|(r, c)| subbox(&board, r, c))
            && (0..9).all(|i| row(&board, i) && col(&board, i))
    }
}
