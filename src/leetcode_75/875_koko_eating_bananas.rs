impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let f = |s: i64| -> bool {
            piles
                .iter() //
                .map(|&p| (p as i64 + s - 1) / s)
                .sum::<i64>()
                <= h as i64
        };
        let (mut left, mut right) = (0, piles.iter().max().unwrap() + 1);
        while left + 1 < right {
            let mid = (left + right) / 2;
            if f(mid as i64) {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    }
}
