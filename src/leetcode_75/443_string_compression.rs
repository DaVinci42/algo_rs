impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut res = Vec::with_capacity(chars.len());
        for c in chars.iter() {
            match res.last() {
                Some(&(cc, n)) if cc == c => {
                    res.pop();
                    res.push((c, n + 1));
                }
                _ => res.push((c, 1)),
            }
        }

        let res = res
            .iter() //
            .fold(String::new(), |mut acc, &(&c, n)| {
                acc.push(c);
                if n > 1 {
                    acc.push_str(&n.to_string());
                }
                acc
            });

        for (i, c) in res.chars().enumerate() {
            chars[i] = c;
        }
        res.len() as i32
    }
}
