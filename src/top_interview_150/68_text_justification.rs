impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut res: Vec<(Vec<&str>, usize)> = vec![];
        let (mut cur, mut char_sum) = (vec![], 0);
        for w in words.iter() {
            if char_sum + cur.len() + w.len() <= max_width {
                cur.push(w.as_str());
                char_sum += w.len();
            } else {
                res.push((cur, char_sum));
                (cur, char_sum) = (vec![w.as_str()], w.len());
            }
        }
        res.push((cur, char_sum));

        res.iter() //
            .enumerate()
            .map(|(i, (v, char_sum))| {
                if v.len() == 1 || i == res.len() - 1 {
                    let mut s = v.join(" ");
                    s.push_str(&" ".repeat(max_width - s.len()));
                    s
                } else {
                    let total_space = max_width - char_sum;
                    let space_count = v.len() - 1;
                    let base = total_space / space_count;
                    let mut extra = (total_space % space_count) as i32;
                    let mut s = String::new();
                    for &w in v.iter() {
                        if s.is_empty() {
                            s.push_str(w);
                        } else {
                            s.push_str(&" ".repeat(if extra > 0 { base + 1 } else { base }));
                            s.push_str(w);
                            extra -= 1;
                        }
                    }
                    s
                }
            })
            .collect()
    }
}
