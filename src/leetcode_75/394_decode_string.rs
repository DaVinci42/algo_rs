use std::vec;

impl Solution {
    // "3[a]2[bc]"
    pub fn decode_string(s: String) -> String {
        let mut res: Vec<char> = vec![];
        for c in s.chars() {
            if c != ']' {
                res.push(c);
                continue;
            }
            let mut tmp = vec![];
            while res.last().unwrap_or(&' ') != &'[' {
                tmp.push(res.pop().unwrap());
            }
            tmp.reverse();
            // pop '['
            res.pop();
            let mut count = vec![];
            while res.last().unwrap_or(&' ').is_ascii_digit() {
                count.push(res.pop().unwrap());
            }
            let count = count
                .iter() //
                .rev()
                .fold(0, |acc, e| acc * 10 + e.to_digit(10).unwrap());
            for _ in 0..count {
                res.extend(&tmp);
            }
        }
        res.iter().collect()
    }
}
