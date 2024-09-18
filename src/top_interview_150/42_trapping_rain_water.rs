impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut left_vec = vec![0; height.len()];
        for (i, &h) in height.iter().enumerate() {
            cur = cur.max(h);
            left_vec[i] = cur;
        }
        let mut right_vec = vec![0; height.len()];
        cur = 0;
        for (i, &h) in height.iter().enumerate().rev() {
            cur = cur.max(h);
            right_vec[i] = cur;
        }
        (0..height.len())
            .map(|i| (left_vec[i].min(right_vec[i]) - height[i]).max(0))
            .sum()
    }
}
