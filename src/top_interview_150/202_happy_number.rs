use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut visited = HashSet::new();
        while n != 1 {
            if !visited.insert(n) {
                return false;
            }
            let mut s = 0;
            while n != 0 {
                let d = n % 10;
                s += d * d;
                n /= 10;
            }
            n = s;
        }
        true
    }
}
