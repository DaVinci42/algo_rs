use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut max_sum, mut max_level, mut cur_level) = (i32::MIN, 0, 0);
        let mut deque = VecDeque::from([root]);
        while !deque.is_empty() {
            cur_level += 1;
            let mut cur_sum = 0;
            for _ in 0..deque.len() {
                let node = deque.pop_front().unwrap().unwrap();
                let node = node.borrow();
                cur_sum += node.val;
                if let Some(left) = &node.left {
                    deque.push_back(Some(Rc::clone(left)));
                }
                if let Some(right) = &node.right {
                    deque.push_back(Some(Rc::clone(right)));
                }
            }
            if cur_sum > max_sum {
                max_sum = cur_sum;
                max_level = cur_level;
            }
        }
        max_level
    }
}
