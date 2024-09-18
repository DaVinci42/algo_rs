impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn search(left: usize, right: usize, nums: &[i32], target: i32) -> i32 {
            let (mut left, mut right) = (left as i32 - 1, right as i32 + 1);
            while left + 1 < right {
                let mid = (left + right) / 2;
                if nums[mid as usize] <= target {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            if left == -1 || nums[left as usize] == target {
                left
            } else {
                -1
            }
        }

        let last = *nums.last().unwrap();
        if nums[0] <= last {
            return search(0, nums.len() - 1, &nums, target);
        }

        let (mut left, mut right) = (-1, nums.len() as i32);
        while left + 1 < right {
            let mid = (left + right) / 2;
            if nums[mid as usize] >= nums[0] {
                left = mid;
            } else {
                right = mid;
            }
        }
        let right = right as usize;
        if target >= nums[0] {
            search(0, right - 1, &nums, target)
        } else {
            search(right, nums.len() - 1, &nums, target)
        }
    }
}
