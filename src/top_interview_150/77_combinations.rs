impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(cur: &mut Vec<i32>, i: usize, max: usize, k: usize, res: &mut Vec<Vec<i32>>) {
            if cur.len() > k {
                return;
            }
            if cur.len() == k {
                res.push(cur.to_vec());
                return;
            }
            if i > max {
                return;
            }

            backtrack(cur, i + 1, max, k, res);
            cur.push(i as i32);
            backtrack(cur, i + 1, max, k, res);
            cur.pop();
        }

        let mut res = vec![];
        backtrack(&mut vec![], 1, n as usize, k as usize, &mut res);
        res
    }
}
