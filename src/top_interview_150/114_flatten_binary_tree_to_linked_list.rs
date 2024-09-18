type LeetNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn dfs(node: &LeetNode) -> (LeetNode, LeetNode) {
        if let Some(node) = node {
            let (head, mut tail) = (node.clone(), node.clone());
            let (l_head, l_tail) = Self::dfs(&node.borrow().left);
            let (r_head, r_tail) = Self::dfs(&node.borrow().right);
            {
                node.borrow_mut().left = None;
            }
            if let Some(l_head) = l_head {
                tail.borrow_mut().right = Some(l_head);
                tail = l_tail.unwrap_or(tail);
            }
            if let Some(r_head) = r_head {
                tail.borrow_mut().right = Some(r_head);
                tail = r_tail.unwrap_or(tail);
            }
            (Some(head), Some(tail))
        } else {
            (None, None)
        }
    }

    pub fn flatten(root: &mut LeetNode) {
        Self::dfs(root);
    }
}
