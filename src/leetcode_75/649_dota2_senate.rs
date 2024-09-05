use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut deque: VecDeque<_> = senate.chars().collect();
        let (mut ban_r, mut ban_d) = (0, 0);
        let (mut count_r, mut count_d) = (
            senate.chars().filter(|&c| c == 'R').count(),
            senate.chars().filter(|&c| c == 'D').count(),
        );

        while count_r > 0 && count_d > 0 {
            match deque.pop_front().unwrap() {
                'R' => {
                    if ban_r > 0 {
                        ban_r -= 1;
                        count_r -= 1;
                    } else {
                        ban_d += 1;
                        deque.push_back('R');
                    }
                }
                _ => {
                    if ban_d > 0 {
                        ban_d -= 1;
                        count_d -= 1;
                    } else {
                        ban_r += 1;
                        deque.push_back('D');
                    }
                }
            }
        }
        if count_r == 0 {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}
