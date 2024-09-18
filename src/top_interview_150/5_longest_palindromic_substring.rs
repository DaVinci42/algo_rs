impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn dp(s: &[u8], mut left: usize, mut right: usize, res: &mut (usize, usize)) {
            while left >= 1 && s.get(left - 1) == s.get(right + 1) {
                (left, right) = (left - 1, right + 1);
            }
            if right - left >= res.1 - res.0 {
                *res = (left, right);
            }
        }

        let s = s.as_bytes();
        let mut res = (0, 0);
        for i in 0..s.len() {
            dp(s, i, i, &mut res);
            if s.get(i) == s.get(i + 1) {
                dp(s, i, i + 1, &mut res);
            }
        }

        s[res.0..=res.1].iter().map(|&b| b as char).collect()
    }
}
