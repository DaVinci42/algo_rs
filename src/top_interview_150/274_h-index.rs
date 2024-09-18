impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        for (i, &c) in citations.iter().enumerate() {
            let count = (citations.len() - i) as i32;
            if c >= count {
                return count;
            }
        }
        0
    }
}
