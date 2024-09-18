impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut c5 = 0;
        for i in 1..=n {
            let mut i = i;
            while i % 5 == 0 {
                c5 += 1;
                i /= 5;
            }
        }
        c5
    }
}
