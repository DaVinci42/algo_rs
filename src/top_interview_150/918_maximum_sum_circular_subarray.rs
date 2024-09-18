use std::cmp;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut pre_min, mut cur_sum, mut max_sub) = (0, 0, i32::MIN);
        for &n in nums.iter() {
            cur_sum += n;
            max_sub = max_sub.max(cur_sum - pre_min);
            pre_min = pre_min.min(cur_sum);
        }

        fn minsum(nums: &[i32]) -> i32 {
            if nums.is_empty() {
                return i32::MAX;
            }
            let (mut pre_max, mut cur_sum, mut min_sub) = (0, 0, i32::MAX);
            for &n in nums.iter() {
                cur_sum += n;
                min_sub = min_sub.min(cur_sum - pre_max);
                pre_max = pre_max.max(cur_sum);
            }
            min_sub
        }

        if nums.len() == 1 {
            max_sub
        } else {
            let min_sub = cmp::min(
                minsum(&nums[1..]), //
                minsum(&nums[0..(nums.len() - 1)]),
            );
            cmp::max(
                max_sub, //
                nums.iter().sum::<i32>() - min_sub,
            )
        }
    }
}
