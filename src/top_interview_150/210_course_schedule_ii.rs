use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut graph = vec![vec![]; num_courses];
        let mut counter = vec![0; num_courses];

        for p in prerequisites.iter() {
            let (u, v) = (p[0] as usize, p[1] as usize);
            graph[v].push(u);
            counter[u] += 1;
        }
        let mut deque: VecDeque<_> = counter
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 0 { Some(i) } else { None })
            .collect();

        let mut res = Vec::with_capacity(num_courses);
        while let Some(u) = deque.pop_front() {
            res.push(u as i32);
            for &v in graph[u].iter() {
                counter[v] -= 1;
                if counter[v] == 0 {
                    deque.push_back(v);
                }
            }
        }
        if res.len() == num_courses {
            res
        } else {
            vec![]
        }
    }
}
