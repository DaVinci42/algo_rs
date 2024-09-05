use std::collections::HashMap;

struct Trie(bool, HashMap<char, Trie>);

impl Trie {
    fn new() -> Self {
        Self(false, HashMap::new())
    }

    fn insert(&mut self, word: &str) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.1.entry(c).or_insert(Trie::new());
        }
        cur.0 = true;
    }

    fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut cur = self;
        for c in prefix.chars() {
            if let Some(trie) = cur.1.get(&c) {
                cur = trie;
            } else {
                return vec![];
            }
        }

        fn dfs(trie: &Trie, path: &str, res: &mut Vec<String>) {
            if res.len() >= 3 {
                return;
            }

            if trie.0 {
                res.push(path.to_string());
            }
            for c in 'a'..='z' {
                if let Some(trie) = trie.1.get(&c) {
                    let mut path = path.to_string();
                    path.push(c);
                    dfs(trie, &path, res);
                }
            }
        }

        let mut res = vec![];
        if cur.0 {
            res.push(prefix.to_string());
        }
        for c in 'a'..='z' {
            if let Some(trie) = cur.1.get(&c) {
                let mut path = prefix.to_string();
                path.push(c);
                dfs(trie, &path, &mut res);
            }
        }
        res
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for s in products.iter() {
            trie.insert(s);
        }

        (0..search_word.len())
            .map(|i| trie.starts_with(&search_word[0..=i]))
            .collect()
    }
}
