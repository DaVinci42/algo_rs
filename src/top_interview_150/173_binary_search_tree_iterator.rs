use std::{cell::RefCell, rc::Rc};

struct BSTIterator {
    items: Vec<i32>,
}

impl BSTIterator {
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = node {
            let node = node.borrow();
            let mut res = Self::inorder(node.left.clone());
            res.push(node.val);
            res.extend(Self::inorder(node.right.clone()).iter());
            res
        } else {
            vec![]
        }
    }

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut items = Self::inorder(root);
        items.reverse();
        Self { items }
    }

    fn next(&mut self) -> i32 {
        self.items.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.items.is_empty()
    }
}
