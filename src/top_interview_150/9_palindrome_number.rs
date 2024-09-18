impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            a if a < 0 => false,
            0 => true,
            _ => {
                let s = x.to_string();
                s.as_bytes()
                    .iter()
                    .zip(s.as_bytes().iter().rev())
                    .all(|a| a.0 == a.1)
            }
        }
    }
}
