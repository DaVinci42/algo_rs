const MAPPING: [&str; 10] = [
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digits: Vec<usize> = digits
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let mut res: Vec<_> = MAPPING[digits[0]].chars().map(String::from).collect();
        for i in 1..digits.len() {
            let mut next_res = vec![];
            for c in MAPPING[digits[i]].chars().map(String::from) {
                for pre in res.iter() {
                    next_res.push(pre.clone() + &c);
                }
            }
            res = next_res;
        }
        res
    }
}
