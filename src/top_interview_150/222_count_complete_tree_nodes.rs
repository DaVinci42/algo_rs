use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            1 + Self::count_nodes(node.left.clone()) + Self::count_nodes(node.right.clone())
        } else {
            0
        }
    }
}
