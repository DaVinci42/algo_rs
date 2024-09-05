impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|v| v[1]);
        let (mut cur_left, mut res) = (i32::MIN, 0);
        for v in intervals.iter() {
            let (s, e) = (v[0], v[1]);
            if s < cur_left {
                res += 1;
            } else {
                cur_left = e;
            }
        }
        res
    }
}
