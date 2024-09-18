use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mapping = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        s.chars()
            .fold(vec![], |mut acc, c| {
                match (acc.last(), mapping.get(&c)) {
                    (Some(a), Some(b)) if a == b => {
                        acc.pop();
                    }
                    _ => {
                        acc.push(c);
                    }
                }
                acc
            })
            .is_empty()
    }
}
