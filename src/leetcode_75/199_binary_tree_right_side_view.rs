use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        let mut deque = VecDeque::from([root]);
        while !deque.is_empty() {
            if let Some(node) = deque.back().unwrap() {
                let node = node.borrow();
                res.push(node.val);
            }

            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap().unwrap();
                let node = node.borrow();
                if let Some(left) = &node.left {
                    deque.push_back(Some(Rc::clone(left)));
                }
                if let Some(right) = &node.right {
                    deque.push_back(Some(Rc::clone(right)));
                }
            }
        }
        res
    }
}
