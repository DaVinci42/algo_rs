impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            i: usize,
            cur: &mut Vec<i32>,
            cursum: i32,
            target: i32,
            candidates: &[i32],
            res: &mut Vec<Vec<i32>>,
        ) {
            if cursum == target {
                res.push(cur.to_vec());
            } else if i >= candidates.len() || cursum > target {
                return;
            } else {
                backtrack(i + 1, cur, cursum, target, candidates, res);
                let n = candidates[i];
                cur.push(n);
                backtrack(i, cur, cursum + n, target, candidates, res);
                cur.pop();
            }
        }

        let mut res = vec![];
        backtrack(
            0,
            &mut Vec::with_capacity(candidates.len()),
            0,
            target,
            &candidates,
            &mut res,
        );
        res
    }
}
