impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut right, mut cur_sum, mut res) = (0, 0, i32::MAX);
        for (left, &n) in nums.iter().enumerate() {
            while right < nums.len() && cur_sum < target {
                cur_sum += nums[right];
                right += 1;
            }
            if cur_sum >= target {
                res = res.min((right - left) as i32);
            }
            cur_sum -= n;
        }
        if res == i32::MAX {
            0
        } else {
            res
        }
    }
}
