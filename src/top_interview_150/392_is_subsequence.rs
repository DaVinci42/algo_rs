impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut pos = 0;
        for b in t.iter() {
            if s.get(pos) == Some(b) {
                pos += 1;
            }
            if pos >= s.len() {
                return true;
            }
        }
        pos >= s.len()
    }
}
