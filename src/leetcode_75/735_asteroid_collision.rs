use std::cmp;

impl Solution {
    // positive goes right, negative left
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for &a in asteroids.iter() {
            let mut cur = a;
            loop {
                match (res.last(), cur) {
                    (_, 0) => break,
                    (Some(&pre), _) if pre > 0 && cur < 0 => match pre.cmp(&cur.abs()) {
                        cmp::Ordering::Less => {
                            res.pop();
                        }
                        cmp::Ordering::Equal => {
                            res.pop();
                            cur = 0;
                        }
                        cmp::Ordering::Greater => {
                            cur = 0;
                        }
                    },
                    _ => {
                        res.push(cur);
                        cur = 0;
                    }
                }
            }
        }
        res
    }
}
