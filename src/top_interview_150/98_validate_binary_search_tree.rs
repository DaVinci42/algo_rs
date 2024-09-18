use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(node: Rc<RefCell<TreeNode>>, lower: i64, upper: i64) -> bool {
        let node = node.borrow();
        let val = node.val as i64;
        if val <= lower || val >= upper {
            return false;
        }
        let mut res = true;
        if let Some(left) = node.left.clone() {
            res &= Self::dfs(left, lower, val);
        }
        if let Some(right) = node.right.clone() {
            res &= Self::dfs(right, val, upper);
        }
        res
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root.unwrap(), i64::MIN, i64::MAX)
    }
}
