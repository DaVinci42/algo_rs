impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut pre_min_sum, mut cur, mut res) = (0, 0, i32::MIN);
        for &n in nums.iter() {
            cur += n;
            res = res.max(cur - pre_min_sum);
            pre_min_sum = pre_min_sum.min(cur);
        }
        res
    }
}
