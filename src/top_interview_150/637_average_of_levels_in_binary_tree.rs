use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];
        let mut deque = VecDeque::from([root]);
        while !deque.is_empty() {
            let (mut sum, mut count) = (0_i64, 0);
            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap();
                let node = node.unwrap();
                let node = node.borrow();
                (sum, count) = (sum + node.val as i64, count + 1);

                if node.left.is_some() {
                    deque.push_back(node.left.clone());
                }
                if node.right.is_some() {
                    deque.push_back(node.right.clone());
                }
            }
            res.push(sum as f64 / count as f64);
        }
        res
    }
}
