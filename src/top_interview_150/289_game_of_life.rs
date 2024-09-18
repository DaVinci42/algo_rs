impl Solution {
    // 10: live -> die
    // 11: die -> live
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let neighbour: Vec<_> = (-1..=1)
            .flat_map(|r| {
                (-1..=1) //
                    .map(move |c| (r, c))
            })
            .filter(|&(r, c)| r != 0 || c != 0)
            .collect();
        let (row, col) = (board.len(), board[0].len());
        (0..row)
            .flat_map(|r| (0..col).map(move |c| (r, c)))
            .for_each(|(r, c)| {
                let (m, n) = (r as i32, c as i32);
                let live = neighbour
                    .iter()
                    .map(|&(a, b)| (a + m, b + n))
                    .filter(|(r, c)| {
                        (0..row as i32).contains(r)
                            && (0..col as i32).contains(c)
                            && (board[*r as usize][*c as usize] == 1
                                || board[*r as usize][*c as usize] == 10)
                    })
                    .count();

                if board[r][c] == 0 && live == 3 {
                    board[r][c] = 11;
                } else if board[r][c] == 1 && (live != 2 && live != 3) {
                    board[r][c] = 10;
                }
            });
        (0..row)
            .flat_map(|r| (0..col).map(move |c| (r, c)))
            .for_each(|(r, c)| {
                board[r][c] = match board[r][c] {
                    10 => 0,
                    11 => 1,
                    a => a,
                }
            });
    }
}
