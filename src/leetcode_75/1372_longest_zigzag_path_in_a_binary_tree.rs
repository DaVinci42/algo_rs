use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

type LeetNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn longest_zig_zag(root: LeetNode) -> i32 {
        fn dfs(node: &LeetNode, should_left: bool, cur_steps: i32) -> i32 {
            if let Some(inner_node) = node {
                let inner_node = inner_node.borrow();
                if should_left {
                    cur_steps //
                        .max(dfs(&inner_node.left, false, cur_steps + 1))
                        .max(dfs(&inner_node.right, true, 1))
                } else {
                    cur_steps
                        .max(dfs(&inner_node.right, true, cur_steps + 1))
                        .max(dfs(&inner_node.left, false, 1))
                }
            } else {
                0
            }
        }

        cmp::max(
            dfs(&root, true, 0), //
            dfs(&root, false, 0),
        )
    }
}
