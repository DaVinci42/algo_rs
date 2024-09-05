impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut right = -1_i32;
        let mut res = 0;
        let mut flipped = 0;
        for left in 0..nums.len() {
            while right + 1 < nums.len() as i32 {
                if nums[(right + 1) as usize] == 1 {
                    right += 1;
                } else if flipped < k {
                    right += 1;
                    flipped += 1;
                } else {
                    break;
                }
            }

            res = res.max(right - left as i32 + 1);
            if nums[left] == 0 {
                flipped = 0.max(flipped - 1);
            }
            right = right.max(left as i32);
        }
        res
    }
}
