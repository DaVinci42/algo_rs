use std::collections::HashMap;

struct Trie(bool, HashMap<char, Trie>);

impl Trie {
    fn new() -> Self {
        Self(false, HashMap::new())
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars().into_iter() {
            cur = cur.1.entry(c).or_insert(Trie::new());
        }
        cur.0 = true
    }

    fn exist(&self, s: &str, isword: bool) -> bool {
        let mut cur = self;
        for c in s.chars() {
            if let Some(trie) = cur.1.get(&c) {
                cur = trie;
            } else {
                return false;
            }
        }

        !isword || cur.0
    }

    fn search(&self, word: String) -> bool {
        Self::exist(&self, &word, true)
    }

    fn starts_with(&self, prefix: String) -> bool {
        Self::exist(&self, &prefix, false)
    }
}
