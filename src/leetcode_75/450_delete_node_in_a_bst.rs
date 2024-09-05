use std::cell::RefCell;
use std::rc::Rc;

type LeetNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn delete_node(root: LeetNode, key: i32) -> LeetNode {
        fn delete(node: Rc<RefCell<TreeNode>>) -> LeetNode {
            let node = node.borrow();
            match (&node.left, &node.right) {
                (Some(left), None) => Some(left.clone()),
                (None, Some(right)) => Some(right.clone()),
                (None, None) => None,
                (Some(left), Some(right)) => {
                    let mut min_right = Some(right.clone());

                    while let Some(node) = min_right.clone() {
                        if let Some(left) = &node.borrow().left {
                            min_right = Some(left.clone());
                        } else {
                            break;
                        }
                    }

                    if let Some(node) = min_right {
                        let mut node = node.borrow_mut();
                        node.left = Some(left.clone());
                    }
                    Some(right.clone())
                }
            }
        }

        if let Some(node) = &root {
            let res = node.borrow().val.cmp(&key);
            match res {
                std::cmp::Ordering::Equal => delete(node.clone()),
                std::cmp::Ordering::Greater => {
                    let fin_left = Self::delete_node(node.borrow().left.clone(), key);
                    node.borrow_mut().left = fin_left;
                    root
                }
                std::cmp::Ordering::Less => {
                    let fin_right = Self::delete_node(node.borrow().right.clone(), key);
                    node.borrow_mut().right = fin_right;
                    root
                }
            }
        } else {
            None
        }
    }
}
