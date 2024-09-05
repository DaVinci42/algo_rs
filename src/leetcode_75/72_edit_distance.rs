use std::collections::HashMap;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        fn dp(s1: &[u8], s2: &[u8], i1: i32, i2: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
            if i1 < 0 && i2 < 0 {
                return 0;
            } else if i1 < 0 {
                return i2 + 1;
            } else if i2 < 0 {
                return i1 + 1;
            } else if let Some(&res) = cache.get(&(i1, i2)) {
                return res;
            }

            let res = if s1[i1 as usize] == s2[i2 as usize] {
                dp(s1, s2, i1 - 1, i2 - 1, cache)
            } else {
                1 + dp(s1, s2, i1 - 1, i2, cache)
                    .min(dp(s1, s2, i1, i2 - 1, cache))
                    .min(dp(s1, s2, i1 - 1, i2 - 1, cache))
            };
            cache.insert((i1, i2), res);
            res
        }

        let mut cache = HashMap::new();
        dp(
            word1.as_bytes(),
            word2.as_bytes(),
            word1.len() as i32 - 1,
            word2.len() as i32 - 1,
            &mut cache,
        )
    }
}
