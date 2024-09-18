use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_mirror(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (&p, &q) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            _ => {
                let (p, q) = (p.unwrap(), q.unwrap());
                let (p, q) = (p.borrow(), q.borrow());
                if p.val != q.val {
                    false
                } else {
                    Self::is_mirror(p.left.clone(), q.right.clone())
                        && Self::is_mirror(p.right.clone(), q.left.clone())
                }
            }
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            Self::is_mirror(node.left.clone(), node.right.clone())
        } else {
            true
        }
    }
}
