const INVALID: i32 = i32::MAX;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(cur: &mut Vec<i32>, option: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if cur.len() == option.len() {
                res.push(cur.to_vec());
            } else {
                for i in 0..option.len() {
                    if option[i] == INVALID {
                        continue;
                    }
                    let n = option[i];
                    cur.push(n);
                    option[i] = INVALID;
                    backtrack(cur, option, res);
                    cur.pop();
                    option[i] = n;
                }
            }
        }

        let mut res = vec![];
        backtrack(&mut Vec::with_capacity(nums.len()), &mut nums, &mut res);
        res
    }
}
