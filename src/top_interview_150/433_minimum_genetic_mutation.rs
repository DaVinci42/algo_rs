use std::collections::{HashSet, VecDeque};

type GENE = [char; 8];
const NEIGHBOUR: [char; 4] = ['A', 'C', 'G', 'T'];

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        fn gene(s: &str) -> GENE {
            let mut res = [' '; 8];
            for (i, c) in s.char_indices() {
                res[i] = c;
            }
            res
        }
        let bank: HashSet<GENE> = bank
            .iter() //
            .map(|s| gene(s))
            .collect();
        let (start, end) = (gene(&start_gene), gene(&end_gene));
        let (mut deque, mut visited, mut step) =
            (VecDeque::from([start]), HashSet::from([start]), 0);
        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let mut gene = deque.pop_front().unwrap();
                if gene == end {
                    return step;
                }
                for i in 0..8 {
                    for &c in NEIGHBOUR.iter() {
                        let pre = gene[i];
                        gene[i] = c;
                        if !visited.contains(&gene) && bank.contains(&gene) {
                            deque.push_back(gene);
                            visited.insert(gene);
                        }
                        gene[i] = pre;
                    }
                }
            }
            step += 1
        }
        -1
    }
}