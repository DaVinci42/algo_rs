impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut counter = [0; 32];
        for &n in nums.iter() {
            for i in 0..32 {
                counter[i] += (n >> i) & 1;
            }
        }
        counter.iter().enumerate().fold(
            0,
            |acc, (i, &e)| {
                if e % 3 != 0 {
                    acc | (1 << i)
                } else {
                    acc
                }
            },
        )
    }
}
