impl Solution {
    fn pow(base: f64, n: i64) -> f64 {
        if n == 1 {
            return base;
        }
        let half = Self::pow(base, n / 2);
        if n % 2 == 1 {
            half * half * base
        } else {
            half * half
        }
    }

    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        } else if n == 1 {
            return x;
        }
        let n = n as i64;
        let (is_neg, n) = (n > 0, n.abs());
        if is_neg {
            1.0 / Self::pow(x, n)
        } else {
            Self::pow(x, n)
        }
    }
}
