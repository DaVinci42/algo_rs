use std::collections::HashMap;

struct WordDictionary(bool, HashMap<char, WordDictionary>);

impl WordDictionary {
    fn new() -> Self {
        Self(false, HashMap::new())
    }

    fn add_word(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.1.entry(c).or_insert(WordDictionary::new());
        }
        cur.0 = true
    }

    fn s(&self, w: &str) -> bool {
        match w.chars().next() {
            Some('.') => self.1.values().any(|d| d.s(&w[1..])),
            Some(c) => self.1.get(&c).map_or(false, |wd| wd.s(&w[1..])),
            None => self.0,
        }
    }

    fn search(&self, word: String) -> bool {
        self.s(&word)
    }
}
