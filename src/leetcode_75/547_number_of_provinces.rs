impl Solution {
    pub fn find_circle_num(mut is_connected: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: usize, graph: &mut Vec<Vec<i32>>) {
            graph[i][i] = 0;
            for j in 0..graph.len() {
                if i == j || graph[i][j] == 0 {
                    continue;
                }
                graph[i][j] = 0;
                graph[j][i] = 0;
                dfs(j, graph);
            }
        }

        let (mut res, n) = (0, is_connected.len());
        for i in 0..n {
            if (0..n).any(|j| is_connected[i][j] == 1) {
                dfs(i, &mut is_connected);
                res += 1
            }
        }
        res
    }
}
