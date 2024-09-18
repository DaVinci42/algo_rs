use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut right, mut res) = (0, 0);
        let mut map: HashMap<u8, i32> = HashMap::new();
        let s = s.as_bytes();
        for (left, c) in s.iter().enumerate() {
            while right < s.len() && *map.get(&s[right]).unwrap_or(&0) == 0 {
                map.insert(s[right], 1);
                right += 1;
            }
            res = res.max((right - left) as i32);
            map.remove(c);
        }
        res
    }
}
