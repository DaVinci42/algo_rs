impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(digits.len() + 1);
        res.push(0);
        for &d in digits.iter() {
            res.push(d);
        }
        let mut carry = true;
        for i in (0..res.len()).rev() {
            if carry {
                res[i] += 1;
            }
            carry = res[i] > 9;
            res[i] %= 10;
        }
        if res[0] != 0 {
            res
        } else {
            res.iter().skip(1).cloned().collect()
        }
    }
}
