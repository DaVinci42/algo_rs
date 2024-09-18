use std::collections::{HashMap, HashSet};

struct UnionFind<'a> {
    id: HashMap<&'a str, (&'a str, f64)>,
    size: HashMap<&'a str, usize>,
}

impl<'a> UnionFind<'a> {
    fn new(items: &'a HashSet<&'a str>) -> Self {
        Self {
            id: items.iter().map(|&s| (s, (s, 1.0))).collect(),
            size: items.iter().map(|&s| (s, 1)).collect(),
        }
    }

    fn find(&mut self, p: &'a str) -> (&'a str, f64) {
        let (p_id, p_factor) = self.id[p];
        if p_id != self.id[p_id].0 {
            let (root, factor) = self.find(p_id);
            self.id.insert(p, (root, factor * p_factor));
        }
        self.id[p]
    }

    fn union(&mut self, p: &'a str, q: &'a str, factor: f64) {
        let (p_root, p_factor) = self.find(p);
        let (q_root, q_factor) = self.find(q);
        if p_root == q_root {
            return;
        }

        let (p_size, q_size) = (self.size[p_root], self.size[q_root]);
        if p_size <= q_size {
            self.id
                .insert(p_root, (q_root, factor * q_factor / p_factor));
            self.size.insert(q_root, q_size + p_size);
        } else {
            self.id
                .insert(q_root, (p_root, p_factor / (factor * q_factor)));
            self.size.insert(p_root, p_size + q_size);
        }
    }
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let items: HashSet<&str> = equations
            .iter()
            .flat_map(|v| v.iter().map(|s| s.as_str()))
            .collect();
        let mut uf = UnionFind::new(&items);
        for (i, v) in equations.iter().enumerate() {
            let (a, b, f) = (v[0].as_str(), v[1].as_str(), values[i]);
            uf.union(a, b, f);
        }

        queries
            .iter()
            .map(|v| {
                let (a, b) = (&v[0], &v[1]);
                if !uf.id.contains_key(&a.as_str()) || !uf.id.contains_key(&b.as_str()) {
                    return -1.0;
                }
                let (a_root, a_factor) = uf.find(a);
                let (b_root, b_factor) = uf.find(b);
                if a_root != b_root {
                    -1.0
                } else {
                    a_factor / b_factor
                }
            })
            .collect()
    }
}
