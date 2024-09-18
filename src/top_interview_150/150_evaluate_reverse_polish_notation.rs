use std::collections::HashSet;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut cur = vec![];
        let ops = HashSet::from(["+", "-", "*", "/"]);
        for t in tokens.iter() {
            if ops.contains(t.as_str()) {
                let b = cur.pop().unwrap();
                let a = cur.pop().unwrap();
                match t.as_str() {
                    "+" => cur.push(a + b),
                    "-" => cur.push(a - b),
                    "*" => cur.push(a * b),
                    "/" => cur.push(a / b),
                    _ => unreachable!(),
                }
            } else {
                cur.push(t.parse().unwrap());
            }
        }
        cur[0]
    }
}
