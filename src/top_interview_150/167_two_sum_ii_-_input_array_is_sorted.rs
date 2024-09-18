impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            match target.cmp(&(numbers[left] + numbers[right])) {
                std::cmp::Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
                std::cmp::Ordering::Greater => left += 1,
                std::cmp::Ordering::Less => right -= 1,
            }
        }
        unreachable!()
    }
}
