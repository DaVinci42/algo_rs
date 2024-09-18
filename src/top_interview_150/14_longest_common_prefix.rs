impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        for (i, b) in strs[0].as_bytes().iter().enumerate() {
            if !(1..strs.len()) //
                .all(|j| matches!(strs[j].as_bytes().get(i), Some(cc) if cc == b))
            {
                return strs[0][..i].to_string();
            }
        }
        strs[0].to_string()
    }
}
