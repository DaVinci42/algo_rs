use std::{cmp, str::FromStr};

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        (1..=cmp::min(str1.len(), str2.len()))
            .rev()
            .find_map(|size| {
                if str1.len() % size != 0 || str2.len() % size != 0 {
                    return None;
                }
                if str1[..size] != str2[..size]
                    || str1[..size].repeat(str1.len() / size) != str1
                    || str2[..size].repeat(str2.len() / size) != str2
                {
                    return None;
                }
                Some(String::from_str(&str1[..size]).unwrap())
            })
            .unwrap_or(String::from(""))
    }
}
