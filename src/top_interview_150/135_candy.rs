use std::cmp;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy[i] = cmp::max(candy[i], candy[i - 1] + 1);
            }
        }
        for i in (1..ratings.len()).rev() {
            if ratings[i - 1] > ratings[i] {
                candy[i - 1] = cmp::max(candy[i - 1], candy[i] + 1);
            }
        }
        candy.iter().sum()
    }
}
