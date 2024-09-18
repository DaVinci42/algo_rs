use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut borrow_node = node.borrow_mut();
            (borrow_node.left, borrow_node.right) = (
                Self::invert_tree(borrow_node.right.clone()), //
                Self::invert_tree(borrow_node.left.clone()),
            );
            Some(node.clone())
        } else {
            None
        }
    }
}
