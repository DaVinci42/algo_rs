impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            _ => {
                let (mut a, mut b, mut c) = (0, 1, 1);
                for _ in 0..n - 2 {
                    (a, b, c) = (b, c, a + b + c);
                }
                c
            }
        }
    }
}
