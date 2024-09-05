use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let node = node.borrow();
            match node.val.cmp(&val) {
                cmp::Ordering::Less => Self::search_bst(node.right.clone(), val),
                cmp::Ordering::Equal => root.clone(),
                cmp::Ordering::Greater => Self::search_bst(node.left.clone(), val),
            }
        } else {
            None
        }
    }
}
