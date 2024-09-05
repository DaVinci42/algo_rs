impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut cur: Vec<(i32, usize)> = vec![];
        for &n in nums.iter() {
            match cur.last() {
                Some(&(pre, num)) if pre == n => {
                    cur.pop();
                    cur.push((n, num + 1));
                }
                _ => cur.push((n, 1)),
            }
        }

        if cur.len() == 1 {
            let (n, size) = cur[0];
            return if n == 0 { 0 } else { size as i32 - 1 };
        }

        let mut res = 0;
        for (i, &(b, size)) in cur.iter().enumerate() {
            match (i, b, size) {
                (_, 1, _) => res = res.max(size),
                (0, 0, 1) => res = res.max(cur.get(1).map(|t| t.1).unwrap_or_default()),
                (i, 0, 1) => {
                    res = res.max(
                        cur.get(i - 1).map(|t| t.1).unwrap_or_default()
                            + cur.get(i + 1).map(|t| t.1).unwrap_or_default(),
                    )
                }
                _ => {}
            }
        }
        res as i32
    }
}
