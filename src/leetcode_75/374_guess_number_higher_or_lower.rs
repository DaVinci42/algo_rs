impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (0_i64, n as i64 + 1);
        while left + 1 < right {
            let mid = (left + right) / 2;
            match guess(mid as i32) {
                0 => return mid as i32,
                -1 => right = mid,
                _ => left = mid,
            }
        }
        unreachable!();
    }
}
