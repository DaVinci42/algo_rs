impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut cur: i64 = nums[..k].iter().map(|&n| n as i64).sum();
        let mut res: i64 = cur;
        for right in k..nums.len() {
            cur += nums[right] as i64;
            cur -= nums[right - k] as i64;
            res = res.max(cur)
        }
        res as f64 / k as f64
    }
}
