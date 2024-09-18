impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];
        let (mut left, mut right) = (-1_i32, nums.len() as i32);
        while left + 1 < right {
            let mid = (left + right) as usize / 2;
            if nums[mid] >= target {
                right = mid as i32;
            } else {
                left = mid as i32;
            }
        }
        if right != nums.len() as i32 && nums[right as usize] == target {
            res[0] = right;
        }
        (left, right) = (-1_i32, nums.len() as i32);
        while left + 1 < right {
            let mid = (left + right) as usize / 2;
            if nums[mid] > target {
                right = mid as i32;
            } else {
                left = mid as i32;
            }
        }
        if left != -1 && nums[left as usize] == target {
            res[1] = left;
        }
        res
    }
}
