impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        let s = s.as_bytes();
        let counter: [usize; 128] = t
            .as_bytes() //
            .iter()
            .fold([0; 128], |mut acc, &e| {
                acc[e as usize] += 1;
                acc
            });
        let mut res: Option<(usize, usize)> = None;
        let mut right = 0;
        let mut cur = [0; 128];
        for left in 0..=s.len() - t.len() {
            while right < s.len() && (0..128).any(|u| cur[u] < counter[u]) {
                cur[s[right] as usize] += 1;
                right += 1;
            }
            if (0..128).all(|u| cur[u] >= counter[u]) {
                match res {
                    Some((a, b)) if b - a < right - left => {}
                    _ => res = Some((left, right)),
                }
            }
            cur[s[left] as usize] -= 1;
        }
        if let Some((left, right)) = res {
            (left..right).map(|i| s[i] as char).collect()
        } else {
            "".to_string()
        }
    }
}
