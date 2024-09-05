use std::cell::RefCell;
use std::rc::Rc;

type LeetNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: LeetNode, p: LeetNode, q: LeetNode) -> LeetNode {
        fn dfs(node: &LeetNode, p: i32, q: i32) -> LeetNode {
            if let Some(inner_node) = node {
                let inner_node = inner_node.borrow();
                if inner_node.val == p || inner_node.val == q {
                    return node.clone();
                }
                match (
                    dfs(&inner_node.left, p, q), //
                    dfs(&inner_node.right, p, q),
                ) {
                    (Some(_), Some(_)) => node.clone(),
                    (Some(r), None) | (None, Some(r)) => Some(r),
                    _ => None,
                }
            } else {
                None
            }
        }
        dfs(
            &root, //
            p.unwrap().borrow().val,
            q.unwrap().borrow().val,
        )
    }
}
