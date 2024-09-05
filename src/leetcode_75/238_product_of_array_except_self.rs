impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let prefix = (0..nums.len()) //
            .fold(Vec::with_capacity(nums.len()), |mut acc, i| {
                if let Some(pre) = acc.last() {
                    acc.push(pre * nums[i - 1]);
                } else {
                    acc.push(1);
                }
                acc
            });
        let suffix = (0..nums.len())
            .rev()
            .fold(vec![1; nums.len()], |mut acc, i| {
                if let Some(pre) = acc.get(i + 1) {
                    acc[i] = pre * nums[i + 1];
                } else {
                    acc[i] = 1;
                }
                acc
            });

        prefix
            .iter()
            .zip(suffix.iter())
            .map(|(p, s)| p * s)
            .collect()
    }
}
