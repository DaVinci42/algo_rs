use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type LeetNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn size(node: &LeetNode, cache: &mut HashMap<i32, usize>) -> usize {
        if let Some(node) = node {
            let node = node.borrow();
            if let Some(&res) = cache.get(&node.val) {
                return res;
            }
            let res = 1 + Self::size(&node.left, cache) + Self::size(&node.right, cache);
            cache.insert(node.val, res);
            res
        } else {
            0
        }
    }

    fn dfs(node: &LeetNode, k: usize, cache: &mut HashMap<i32, usize>) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let left_count = Self::size(&node.left, cache);
            match (left_count + 1).cmp(&k) {
                std::cmp::Ordering::Equal => node.val,
                std::cmp::Ordering::Less => Self::dfs(&node.right, k - left_count - 1, cache),
                std::cmp::Ordering::Greater => Self::dfs(&node.left, k, cache),
            }
        } else {
            unreachable!()
        }
    }

    pub fn kth_smallest(root: LeetNode, k: i32) -> i32 {
        let mut cache = HashMap::new();
        Self::dfs(&root, k as usize, &mut cache)
    }
}
