impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n as usize {
            1 => 1,
            2 => 2,
            _ => {
                let (mut a, mut b) = (1, 2);
                for _ in 0..n - 2 {
                    (a, b) = (b, a + b);
                }
                b
            }
        }
    }
}
