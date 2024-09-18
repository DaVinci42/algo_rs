use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern = pattern.chars();
        let mut s = s.split_whitespace();
        let mut ps_map: HashMap<char, &str> = HashMap::new();
        let mut sp_map: HashMap<&str, char> = HashMap::new();
        loop {
            match (pattern.next(), s.next()) {
                (None, None) => return true,
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(p), Some(w)) => match (ps_map.get(&p), sp_map.get(&w)) {
                    (Some(&ww), Some(&pp)) if pp == p && ww == w => continue,
                    (None, None) => {
                        ps_map.insert(p, w);
                        sp_map.insert(w, p);
                    }
                    _ => return false,
                },
            }
        }
    }
}
