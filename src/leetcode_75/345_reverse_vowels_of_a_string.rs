use std::collections::HashSet;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = "aAeEiIoOuU".chars().collect::<HashSet<_>>();
        let mut res = s.chars().collect::<Vec<_>>();
        let (mut left, mut right) = (0, res.len() - 1);
        while left < right {
            if !vowels.contains(&res[left]) {
                left += 1
            } else if !vowels.contains(&res[right]) {
                right -= 1;
            } else {
                (res[left], res[right]) = (res[right], res[left]);
                (left, right) = (left + 1, right - 1);
            }
        }
        res.iter().collect()
    }
}
