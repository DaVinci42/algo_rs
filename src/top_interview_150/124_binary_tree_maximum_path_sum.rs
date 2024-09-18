use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type LeetNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn nodekey(node: &Rc<RefCell<TreeNode>>) -> usize {
        Rc::as_ptr(node) as usize
    }

    fn max_from_node(node: &LeetNode, cache: &mut HashMap<usize, i32>) -> i32 {
        if let Some(rc) = node {
            let key = Self::nodekey(rc);
            if let Some(&res) = cache.get(&key) {
                return res;
            }
            let node = rc.borrow();
            let res = node
                .val
                .max(node.val + Self::max_from_node(&node.left, cache))
                .max(node.val + Self::max_from_node(&node.right, cache));
            cache.insert(key, res);
            res
        } else {
            0
        }
    }

    fn dfs(node: &LeetNode, cache: &mut HashMap<usize, i32>) -> i32 {
        if let Some(rc) = node {
            let node = rc.borrow();
            let (left, right) = (
                Self::max_from_node(&node.left, cache),
                Self::max_from_node(&node.right, cache),
            );
            (left.max(0) + node.val + right.max(0))
                .max(Self::dfs(&node.left, cache))
                .max(Self::dfs(&node.right, cache))
        } else {
            i32::MIN
        }
    }

    pub fn max_path_sum(root: LeetNode) -> i32 {
        let mut cache = HashMap::new();
        Self::dfs(&root, &mut cache)
    }
}
