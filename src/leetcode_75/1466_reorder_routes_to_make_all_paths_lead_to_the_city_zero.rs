use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for edge in connections.iter() {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push((v, true));
            graph[v].push((u, false));
        }

        let mut res = 0;
        let mut deque = VecDeque::from([0]);
        let mut visited = HashSet::from([0]);
        while let Some(u) = deque.pop_back() {
            for &(v, should_reverse) in graph[u].iter() {
                if visited.contains(&v) {
                    continue;
                }
                if should_reverse {
                    res += 1;
                }
                deque.push_back(v);
                visited.insert(v);
            }
        }
        res
    }
}
