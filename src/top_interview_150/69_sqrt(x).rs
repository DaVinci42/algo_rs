impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let (mut left, mut right) = (0_i64, x + 1);
        while left + 1 < right {
            let mid = (left + right) / 2;
            if mid * mid <= x {
                left = mid;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}
