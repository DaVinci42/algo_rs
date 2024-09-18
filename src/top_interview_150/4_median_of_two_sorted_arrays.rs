impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let (nums1, nums2) = if nums1.len() >= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        fn safe_get(nums: &[i32], i: i32, default: i32) -> i32 {
            if !(0 <= i && i < nums.len() as i32) {
                default
            } else {
                nums[i as usize]
            }
        }
        let (mut left, mut right) = (0, total / 2);
        loop {
            let count1 = (left + right) / 2;
            let count2 = total / 2 - count1;
            if count2 > nums2.len() {
                left = count1 + 1;
                continue;
            }

            let left_n1 = safe_get(&nums1, count1 as i32 - 1, i32::MIN);
            let right_n1 = safe_get(&nums1, count1 as i32, i32::MAX);

            let left_n2 = safe_get(&nums2, count2 as i32 - 1, i32::MIN);
            let right_n2 = safe_get(&nums2, count2 as i32, i32::MAX);

            if right_n1 < left_n2 {
                left = count1 + 1;
            } else if left_n1 > right_n2 {
                right = count1 - 1;
            } else {
                return if total % 2 == 1 {
                    right_n1.min(right_n2) as f64
                } else {
                    (left_n1.max(left_n2) as f64 + right_n1.min(right_n2) as f64) / 2.0
                };
            }
        }
    }
}
