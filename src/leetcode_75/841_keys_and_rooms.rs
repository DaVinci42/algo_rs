use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let (mut deque, mut visited) = (VecDeque::from([0_i32]), HashSet::from([0]));
        while let Some(u) = deque.pop_front() {
            for &v in rooms[u as usize].iter() {
                if visited.contains(&v) {
                    continue;
                }
                deque.push_back(v);
                visited.insert(v);
            }
        }
        visited.len() == rooms.len()
    }
}
