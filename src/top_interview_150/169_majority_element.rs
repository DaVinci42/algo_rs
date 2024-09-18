impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut res, mut count) = (nums[0], 0);
        for &n in nums.iter() {
            if n == res {
                count += 1;
            } else if count > 0 {
                count -= 1;
            } else {
                (res, count) = (n, 1);
            }
        }
        res
    }
}
