impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut pre = vec![i32::MAX; nums.len()];
        for i in 1..nums.len() - 1 {
            pre[i] = nums[i - 1].min(pre[i - 1]);
        }
        let mut post = vec![i32::MIN; nums.len()];
        for i in (1..nums.len() - 1).rev() {
            post[i] = nums[i + 1].max(post[i + 1]);
        }
        for (i, &n) in nums.iter().enumerate() {
            if pre[i] < n && n < post[i] {
                return true;
            }
        }
        false
    }
}
