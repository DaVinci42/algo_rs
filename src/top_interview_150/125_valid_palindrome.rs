impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<_> = s
            .chars()
            .filter_map(|c| {
                if c.is_ascii_alphanumeric() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .collect();
        (0..chars.len() / 2) //
            .all(|i| chars[i] == chars[chars.len() - 1 - i])
    }
}
