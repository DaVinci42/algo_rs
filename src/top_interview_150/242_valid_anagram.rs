impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        fn counter(s: &str) -> [usize; 26] {
            let mut res = [0; 26];
            for b in s.as_bytes().iter() {
                res[(b - b'a') as usize] += 1;
            }
            res
        }

        counter(&s) == counter(&t)
    }
}
