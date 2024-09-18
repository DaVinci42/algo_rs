impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());
        (0..haystack.len())
            .find(|i| {
                (0..needle.len()) //
                    .all(|j| haystack.get(i + j) == needle.get(j))
            })
            .map_or(-1, |u| u as i32)
    }
}
