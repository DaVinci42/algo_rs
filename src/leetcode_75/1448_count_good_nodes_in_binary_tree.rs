use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, cur_max: i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let cur = if node.val >= cur_max { 1 } else { 0 };
                let cur_max = cur_max.max(node.val);
                cur + dfs(node.left.clone(), cur_max) + dfs(node.right.clone(), cur_max)
            } else {
                0
            }
        }

        dfs(root, i32::MIN)
    }
}
