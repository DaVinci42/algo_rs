use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let (mut deque, mut res) = (VecDeque::from([root.unwrap()]), 0);
        while !deque.is_empty() {
            res += 1;
            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap();
                let node = node.borrow();
                if let Some(left) = &node.left {
                    deque.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    deque.push_back(right.clone());
                }
            }
        }
        res
    }
}
