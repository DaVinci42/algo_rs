use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            cur_sum: i64,
            target_sum: i64,
            counter: &mut HashMap<i64, i32>,
            res: &mut i32,
        ) {
            if let Some(node) = node {
                let node = node.borrow();
                let cur_sum = cur_sum + node.val as i64;
                *res += counter.get(&(cur_sum - target_sum)).unwrap_or(&0);
                *counter.entry(cur_sum).or_default() += 1;

                if node.left.is_some() {
                    dfs(node.left.clone(), cur_sum, target_sum, counter, res);
                }
                if node.right.is_some() {
                    dfs(node.right.clone(), cur_sum, target_sum, counter, res);
                }
                // every node should use parent path only
                *counter.entry(cur_sum).or_default() -= 1;
            }
        }

        let mut res = 0;
        let mut counter = HashMap::from([(0, 1)]);
        dfs(root, 0, target_sum as i64, &mut counter, &mut res);
        res
    }
}
