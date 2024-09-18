impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i1, mut i2) = (m - 1, n - 1);
        (0..(m + n) as usize)
            .rev() //
            .for_each(|i| {
                if i1 < 0 || (i2 >= 0 && nums2[i2 as usize] >= nums1[i1 as usize]) {
                    nums1[i] = nums2[i2 as usize];
                    i2 -= 1;
                } else if i2 < 0 || (i1 >= 0 && nums2[i2 as usize] < nums1[i1 as usize]) {
                    nums1[i] = nums1[i1 as usize];
                    i1 -= 1;
                }
            });
    }
}
