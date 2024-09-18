impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort_unstable();
        let mut cur: Vec<Vec<i32>> = vec![];
        for v in intervals.iter() {
            let (s, e) = (v[0], v[1]);
            match cur.last() {
                Some(v) if v[1] >= s => {
                    let pre = cur.pop().unwrap();
                    cur.push(vec![pre[0], pre[1].max(e)]);
                }
                _ => cur.push(vec![s, e]),
            }
        }
        cur
    }
}
