impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        spells
            .iter()
            .map(|&s| {
                let (mut left, mut right) = (-1_i32, potions.len() as i32);
                while left + 1 < right {
                    let mid = (left + right) / 2;
                    if (s as i64) * (potions[mid as usize] as i64) < success {
                        left = mid;
                    } else {
                        right = mid;
                    }
                }
                potions.len() as i32 - right
            })
            .collect()
    }
}
