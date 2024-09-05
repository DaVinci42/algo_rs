use std::collections::HashSet;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowels: HashSet<_> = "aeiou".as_bytes().iter().collect();
        let (k, s) = (k as usize, s.as_bytes());
        let mut cur = s[..k]
            .iter() //
            .filter(|b| vowels.contains(b))
            .count();
        let mut res = cur;
        for right in k..s.len() {
            if vowels.contains(&s[right]) {
                cur += 1;
            }
            if vowels.contains(&s[right - k]) {
                cur -= 1;
            }
            res = res.max(cur);
        }
        res as i32
    }
}
