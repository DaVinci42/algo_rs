impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(cur: &mut Vec<char>, open: usize, target: usize, res: &mut Vec<String>) {
            if cur.len() > target {
                return;
            }
            if cur.len() == target && open == 0 {
                res.push(cur.iter().collect());
            } else {
                if open > 0 {
                    cur.push(')');
                    backtrack(cur, open - 1, target, res);
                    cur.pop();
                }
                cur.push('(');
                backtrack(cur, open + 1, target, res);
                cur.pop();
            }
        }

        let n = n as usize;
        let mut res = vec![];
        backtrack(&mut Vec::with_capacity(n * 2), 0, n * 2, &mut res);
        res
    }
}
