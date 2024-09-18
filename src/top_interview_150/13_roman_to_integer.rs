use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mapping: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let (mut res, mut pre) = (0, 0);
        for c in s.chars() {
            let n = mapping[&c];
            if pre < n {
                res -= pre;
                res += n - pre;
            } else {
                res += n;
            }
            pre = n;
        }
        res
    }
}
