impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let (total, mut left) = (nums.iter().sum::<i32>(), 0);
        for (i, &n) in nums.iter().enumerate() {
            if left == total - left - n {
                return i as i32;
            }
            left += n;
        }
        -1
    }
}
