use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut res = vec![];
        let mut deque = VecDeque::from([root.unwrap()]);
        let mut reverse = false;
        while !deque.is_empty() {
            let mut level = vec![];
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front() {
                    let node = node.borrow();
                    level.push(node.val);
                    if let Some(left) = node.left.clone() {
                        deque.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        deque.push_back(right);
                    }
                }
            }
            if reverse {
                level.reverse();
            }
            reverse = !reverse;
            res.push(level);
        }
        res
    }
}
