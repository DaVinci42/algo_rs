impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        fn count(s: &[u8]) -> [usize; 26] {
            let mut res = [0; 26];
            for b in s.iter() {
                res[(b - b'a') as usize] += 1;
            }
            res
        }
        let (c1, c2) = (count(ransom_note.as_bytes()), count(magazine.as_bytes()));
        (0..26).all(|i| c1[i] <= c2[i])
    }
}
