impl Solution {
    fn cal_bracket(s: &str) -> Vec<char> {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                ' ' => continue,
                ')' => {
                    let mut content = vec![];
                    while matches!(stack.last(), Some(&cc) if cc != '(') {
                        content.push(stack.pop().unwrap());
                    }
                    stack.pop();
                    content.reverse();
                    let s = Self::calculate(content.iter().collect());
                    stack.extend(s.to_string().chars());
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        stack
    }

    fn cal_symbol(s: &[char]) -> Vec<char> {
        let mut cur = vec![];
        for &c in s.iter() {
            match (c, cur.last()) {
                ('+', Some(&'+')) => continue,
                ('+', Some(&'-')) => continue,
                ('-', Some(&'+')) => {
                    cur.pop();
                    cur.push('-');
                }
                ('-', Some(&'-')) => {
                    cur.pop();
                    cur.push('+');
                }
                _ => cur.push(c),
            }
        }
        cur
    }

    fn cal_add_sub(s: &[char]) -> i32 {
        let mut res = 0;
        let mut i = 0;
        while i < s.len() {
            let mut j = if s[i] == '+' || s[i] == '-' { i + 1 } else { i };
            while matches!(s.get(j), Some(c) if c.is_ascii_digit()) {
                j += 1;
            }

            res += s[i..j].iter().collect::<String>().parse::<i32>().unwrap();
            i = j;
        }
        res
    }

    pub fn calculate(s: String) -> i32 {
        let s = Self::cal_bracket(&s);
        let s = Self::cal_symbol(&s);
        Self::cal_add_sub(&s)
    }
}
