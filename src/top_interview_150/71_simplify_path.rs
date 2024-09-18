impl Solution {
    pub fn simplify_path(path: String) -> String {
        let res = path
            .split('/') //
            .fold(vec![], |mut acc, e| match e {
                "" | "." => acc,
                ".." => {
                    acc.pop();
                    acc
                }
                s => {
                    acc.push(s);
                    acc
                }
            });
        res.iter() //
            .fold("/".to_string(), |mut acc, &e| {
                if acc.len() != 1 {
                    acc.push('/');
                }
                acc.push_str(e);
                acc
            })
    }
}
