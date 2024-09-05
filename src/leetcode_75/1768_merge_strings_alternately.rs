impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::with_capacity(word1.len() + word2.len());
        let (mut word1, mut word2) = (word1.chars(), word2.chars());
        loop {
            match (word1.next(), word2.next()) {
                (Some(c1), Some(c2)) => {
                    res.push(c1);
                    res.push(c2);
                }
                (Some(c1), None) => res.push(c1),
                (None, Some(c2)) => res.push(c2),
                (None, None) => break,
            }
        }
        res
    }
}
