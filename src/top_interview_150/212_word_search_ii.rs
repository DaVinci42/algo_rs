use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Trie(bool, HashMap<char, Trie>);

impl Trie {
    fn new() -> Self {
        Self(false, HashMap::new())
    }

    fn insert(&mut self, s: &str) {
        let mut cur = self;
        for c in s.chars() {
            cur = cur.1.entry(c).or_insert(Trie::new());
        }
        cur.0 = true;
    }
}

const NEIGHBOUR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn dfs(
            r: usize,
            c: usize,
            trie: &Trie,
            path: &str,
            res: &mut HashSet<String>,
            board: &mut Vec<Vec<char>>,
            words: &HashSet<&str>,
        ) {
            if trie.0 && words.contains(path) {
                res.insert(path.to_string());
            }
            let pre_char = board[r][c];
            board[r][c] = ' ';
            let (row, col) = (board.len() as i32, board[0].len() as i32);
            for (m, n) in NEIGHBOUR.iter() {
                let (a, b) = (r as i32 + m, c as i32 + n);
                if !(0 <= a && a < row && 0 <= b && b < col) {
                    continue;
                }
                let (r, c) = (a as usize, b as usize);
                let char = board[r][c];
                if let Some(t) = trie.1.get(&char) {
                    let mut path = path.to_string();
                    path.push(char);
                    dfs(r, c, t, &path, res, board, words);
                }
            }
            board[r][c] = pre_char;
        }

        let words: HashSet<&str> = words.iter().map(|s| s.as_str()).collect();
        let mut trie = Trie::new();
        for &w in words.iter() {
            trie.insert(w);
        }

        let mut res = HashSet::new();
        let (row, col) = (board.len(), board[0].len());
        for r in 0..row {
            for c in 0..col {
                let char = board[r][c];
                if let Some(trie) = trie.1.get(&char) {
                    dfs(r, c, trie, &char.to_string(), &mut res, &mut board, &words);
                }
            }
        }
        res.iter().cloned().collect()
    }
}
