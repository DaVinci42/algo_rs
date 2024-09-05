use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn leaf(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            let node = node.unwrap();
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                res.push(node.val);
            } else {
                if let Some(left) = &node.left {
                    leaf(Some(Rc::clone(left)), res);
                }
                if let Some(right) = &node.right {
                    leaf(Some(Rc::clone(right)), res);
                }
            }
        }

        let (mut res1, mut res2) = (vec![], vec![]);
        leaf(root1, &mut res1);
        leaf(root2, &mut res2);
        res1 == res2
    }
}
