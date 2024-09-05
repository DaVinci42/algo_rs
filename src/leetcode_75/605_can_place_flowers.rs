impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
            if n <= 0 {
                break;
            }
            if flowerbed[i] == 1
                || flowerbed.get(i - 1) == Some(&1)
                || flowerbed.get(i + 1) == Some(&1)
            {
                continue;
            }
            flowerbed[i] = 1;
            n -= 1;
        }
        n <= 0
    }
}
