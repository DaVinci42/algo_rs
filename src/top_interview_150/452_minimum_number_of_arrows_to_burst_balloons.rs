impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[1]);
        let (mut farest, mut res) = (i64::MIN, 0);
        for p in points.iter() {
            let (s, e) = (p[0] as i64, p[1] as i64);
            if farest >= s {
                continue;
            }
            res += 1;
            farest = farest.max(e);
        }
        res
    }
}
