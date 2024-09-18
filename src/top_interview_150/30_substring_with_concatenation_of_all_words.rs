use std::collections::HashMap;

impl Solution {
    fn find(s: &[u8], counter: &HashMap<&[u8], usize>, words: &[&[u8]], offset: usize) -> Vec<i32> {
        let mut cur = HashMap::new();
        let (mut res, mut i, mut j) = (vec![], 0, 0);
        let (word_count, word_len) = (words.len(), words[0].len());
        while i + word_count * word_len <= s.len() {
            j = j.max(i);
            while j + word_len <= s.len() {
                let w = &s[j..j + word_len];
                if cur.get(w).unwrap_or(&0) < counter.get(w).unwrap_or(&0) {
                    *cur.entry(w).or_default() += 1;
                    j += word_len;
                } else {
                    break;
                }
            }

            if j - i == word_count * word_len {
                res.push((i + offset) as i32);
            }
            cur.entry(&s[i..i + word_len])
                .and_modify(|v| *v = (*v as i32 - 1).max(0) as usize);
            i += word_len;
        }
        res
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.len() < words.len() * words[0].len() {
            return vec![];
        }
        let counter: HashMap<&[u8], usize> = words
            .iter() //
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(e.as_bytes()).or_default() += 1;
                acc
            });
        let s = s.as_bytes();
        let words: Vec<&[u8]> = words.iter().map(|s| s.as_bytes()).collect();

        (0..words[0].len())
            .flat_map(|i| Self::find(&s[i..], &counter, &words, i))
            .collect()
    }
}
