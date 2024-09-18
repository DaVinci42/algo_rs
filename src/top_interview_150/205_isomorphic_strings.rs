use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut st_mapping: HashMap<u8, u8> = HashMap::new();
        let mut ts_mapping: HashMap<u8, u8> = HashMap::new();
        (0..s.len()).all(|i| {
            let (sc, tc) = (s[i], t[i]);
            if st_mapping.get(&sc).map_or(false, |&c| c != tc) {
                return false;
            }
            if ts_mapping.get(&tc).map_or(false, |&c| c != sc) {
                return false;
            }
            st_mapping.insert(sc, tc);
            ts_mapping.insert(tc, sc);
            true
        })
    }
}
