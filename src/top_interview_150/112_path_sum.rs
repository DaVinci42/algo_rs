use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, mut target_sum: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            target_sum -= node.val;
            if node.left.is_none() && node.right.is_none() {
                return target_sum == 0;
            }
            let mut res = false;
            if let Some(left) = node.left.clone() {
                res |= Self::has_path_sum(Some(left), target_sum);
            }
            if let Some(right) = node.right.clone() {
                res |= Self::has_path_sum(Some(right), target_sum);
            }
            res
        } else {
            false
        }
    }
}
