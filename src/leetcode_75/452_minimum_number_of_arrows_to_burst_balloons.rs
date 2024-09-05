impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[1]);
        let (mut cur_left, mut res) = (i64::MIN, 0);
        for p in points.iter() {
            let (s, e) = (p[0], p[1]);
            if cur_left >= s as i64 {
                continue;
            } else {
                cur_left = e as i64;
                res += 1;
            }
        }
        res
    }
}
