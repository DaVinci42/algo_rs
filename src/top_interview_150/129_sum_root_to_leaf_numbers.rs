use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut cur: i32, res: &mut i32) {
        if let Some(node) = node {
            let node = node.borrow();
            cur = cur * 10 + node.val;
            if node.left.is_none() && node.right.is_none() {
                *res += cur;
            }
            if let Some(left) = node.left.clone() {
                Self::dfs(Some(left), cur, res);
            }
            if let Some(right) = node.right.clone() {
                Self::dfs(Some(right), cur, res);
            }
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::dfs(root, 0, &mut res);
        res
    }
}
