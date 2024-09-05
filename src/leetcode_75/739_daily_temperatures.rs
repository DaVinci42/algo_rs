impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut cur = vec![];
        for (i, &t) in temperatures.iter().enumerate() {
            while let Some(&j) = cur.last() {
                if temperatures[j] < t {
                    cur.pop();
                    res[j] = (i - j) as i32;
                } else {
                    break;
                }
            }
            cur.push(i);
        }
        res
    }
}
