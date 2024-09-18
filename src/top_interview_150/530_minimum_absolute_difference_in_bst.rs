use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(node: Rc<RefCell<TreeNode>>, lower: i32, upper: i32, res: &mut i64) {
        let node = node.borrow();
        *res = (*res)
            .min(upper as i64 - node.val as i64)
            .min(node.val as i64 - lower as i64);
        if let Some(left) = node.left.clone() {
            Self::dfs(left.clone(), lower, node.val, res);
        }
        if let Some(right) = node.right.clone() {
            Self::dfs(right.clone(), node.val, upper, res);
        }
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i64::MAX;
        Self::dfs(root.unwrap(), i32::MIN, i32::MAX, &mut res);
        res as i32
    }
}
