use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut res: Vec<i32> = vec![];
        let mut deque = VecDeque::from([root]);
        while !deque.is_empty() {
            let last = deque.back().unwrap();
            if let Some(last) = last {
                let last = last.borrow();
                res.push(last.val);
            }

            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap();
                if let Some(node) = node {
                    let node = node.borrow();
                    if node.left.is_some() {
                        deque.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        deque.push_back(node.right.clone());
                    }
                }
            }
        }
        res
    }
}
