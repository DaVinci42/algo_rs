use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();
        for s in strs.iter() {
            let mut counter = [0; 26];
            for &b in s.as_bytes().iter() {
                counter[(b - b'a') as usize] += 1;
            }
            map.entry(counter).or_default().push(s.clone());
        }

        map.into_values().collect()
    }
}
