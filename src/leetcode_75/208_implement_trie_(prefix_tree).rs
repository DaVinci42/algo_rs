use std::collections::HashMap;

struct Trie(bool, HashMap<char, Trie>);

impl Trie {
    fn new() -> Self {
        Self(false, HashMap::new())
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.1.entry(c).or_insert(Trie::new());
        }
        cur.0 = true;
    }

    fn find(&self, word: &str, is_word: bool) -> bool {
        let mut cur = self;
        for c in word.chars() {
            if let Some(trie) = cur.1.get(&c) {
                cur = trie;
            } else {
                return false;
            }
        }
        !is_word || cur.0
    }

    fn search(&self, word: String) -> bool {
        self.find(&word, true)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix, false)
    }
}
