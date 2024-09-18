use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            preorder: &[i32],
            pre_left: usize,
            pre_right: usize,
            _inorder: &[i32],
            in_left: usize,
            _in_right: usize,
            parent_index_map: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_left > pre_right || in_left > _in_right {
                return None;
            }
            let parent_val = preorder[pre_left];
            let in_index = parent_index_map[&parent_val];
            let left_size = in_index - in_left;
            let left = if in_index == 0 {
                None
            } else {
                dfs(
                    preorder,
                    pre_left + 1,
                    pre_left + left_size,
                    _inorder,
                    in_left,
                    in_index - 1,
                    parent_index_map,
                )
            };

            Some(Rc::new(RefCell::new(TreeNode {
                val: parent_val,
                left,
                right: dfs(
                    preorder,
                    pre_left + 1 + left_size,
                    pre_right,
                    _inorder,
                    in_index + 1,
                    _in_right,
                    parent_index_map,
                ),
            })))
        }

        let parent_index_map = inorder
            .iter() //
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();

        dfs(
            &preorder,
            0,
            preorder.len() - 1,
            &inorder,
            0,
            inorder.len() - 1,
            &parent_index_map,
        )
    }
}
