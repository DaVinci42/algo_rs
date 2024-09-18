impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = vec![];
        for i in 0..=nums.len() - 3 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let (mut left, mut right, target) = (i + 1, nums.len() - 1, -nums[i]);
            while left < right {
                if left - 1 > i && nums[left] == nums[left - 1] {
                    left += 1;
                    continue;
                }
                match (nums[left] + nums[right]).cmp(&target) {
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                    _ => {
                        res.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                    }
                }
            }
        }
        res
    }
}
