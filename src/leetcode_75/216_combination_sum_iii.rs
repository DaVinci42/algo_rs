impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtracking(
            n: i32,
            pre_sum: i32,
            target_sum: i32,
            pre_num: i32,
            target_num: i32,
            selected: Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if n > 9 || n + pre_sum > target_sum || pre_num + 1 > target_num {
                return;
            }
            if pre_sum + n == target_sum && pre_num + 1 == target_num {
                res.push(selected.iter().chain([n].iter()).cloned().collect());
            }

            // select
            backtracking(
                n + 1,
                pre_sum + n,
                target_sum,
                pre_num + 1,
                target_num,
                selected.iter().chain([n].iter()).cloned().collect(),
                res,
            );

            // skip
            backtracking(
                n + 1,
                pre_sum,
                target_sum,
                pre_num,
                target_num,
                selected,
                res,
            );
        }

        let mut res = vec![];
        backtracking(1, 0, n, 0, k, vec![], &mut res);
        res
    }
}
