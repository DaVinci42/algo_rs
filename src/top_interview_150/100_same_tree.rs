use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (&p, &q) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            _ => {
                let (p, q) = (p.unwrap(), q.unwrap());
                let (p, q) = (p.borrow(), q.borrow());
                if p.val != q.val {
                    false
                } else {
                    Self::is_same_tree(p.left.clone(), q.left.clone())
                        && Self::is_same_tree(p.right.clone(), q.right.clone())
                }
            }
        }
    }
}
