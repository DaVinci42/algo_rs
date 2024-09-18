use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<_> = capital.iter().zip(profits.iter()).collect();
        projects.sort_unstable();
        let (mut heap, mut i) = (BinaryHeap::new(), 0);
        for _ in 0..k {
            while i < projects.len() && *projects[i].0 <= w {
                heap.push(projects[i].1);
                i += 1;
            }
            if let Some(&p) = heap.pop() {
                w += p;
            }
        }
        w
    }
}
