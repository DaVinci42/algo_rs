const MAPPING: [[&str; 10]; 4] = [
    ["", "M", "MM", "MMM", "", "", "", "", "", ""], //
    ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let (a, b, c, d) = (
            (num / 1000) % 10,
            (num / 100) % 10,
            (num / 10) % 10,
            num % 10,
        );

        let mut res = String::new();
        res.push_str(MAPPING[0][a as usize]);
        res.push_str(MAPPING[1][b as usize]);
        res.push_str(MAPPING[2][c as usize]);
        res.push_str(MAPPING[3][d as usize]);
        res
    }
}
