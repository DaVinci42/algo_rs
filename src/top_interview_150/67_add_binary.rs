impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut res = vec!['0'; a.len().max(b.len()) + 1];
        let a: Vec<_> = a.chars().rev().collect();
        let b: Vec<_> = b.chars().rev().collect();

        let mut carry = false;
        res.iter_mut().enumerate().for_each(|(i, n)| {
            let mut cur = 0_usize;
            if carry {
                cur += 1;
                carry = false;
            }
            if matches!(a.get(i), Some('1')) {
                cur += 1;
            }
            if matches!(b.get(i), Some('1')) {
                cur += 1;
            }
            carry = cur > 1;
            *n = if cur % 2 == 0 { '0' } else { '1' }
        });

        if matches!(res.last(), Some('0')) {
            res.pop();
        }
        res.iter().rev().collect()
    }
}
