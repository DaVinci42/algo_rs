impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<u8> = s.bytes().collect();
        let mut i = 0;
        for c in t.bytes() {
            if i >= s.len() {
                return true;
            }
            if s[i] == c {
                i += 1;
            }
        }
        i >= s.len()
    }
}
