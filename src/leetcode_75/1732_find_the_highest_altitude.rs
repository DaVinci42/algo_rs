impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut res, mut cur) = (0, 0);
        for &n in gain.iter() {
            cur += n;
            res = res.max(cur);
        }
        res
    }
}
