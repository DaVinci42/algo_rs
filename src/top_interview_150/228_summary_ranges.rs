impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut cur: Vec<(i32, i32)> = vec![];
        for &n in nums.iter() {
            match cur.last() {
                Some(&(a, b)) if b + 1 == n => {
                    cur.pop();
                    cur.push((a, n));
                }
                _ => cur.push((n, n)),
            }
        }

        cur.iter()
            .map(|&(a, b)| {
                if a == b {
                    a.to_string()
                } else {
                    format!("{}->{}", a, b)
                }
            })
            .collect()
    }
}
