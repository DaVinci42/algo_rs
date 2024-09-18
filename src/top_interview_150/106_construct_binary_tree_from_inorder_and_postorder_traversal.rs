use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type LeetNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> LeetNode {
        fn dfs(
            inorder: &[i32],
            inleft: i32,
            inright: i32,
            postorder: &[i32],
            postleft: i32,
            postright: i32,
            map: &mut HashMap<i32, usize>,
        ) -> LeetNode {
            if !(0 <= inleft && inleft <= inright) {
                return None;
            }
            if !(0 <= postleft && postleft <= postright) {
                return None;
            }
            if inleft > inright || postleft > postright {
                return None;
            }
            let (inleft, inright, postleft, postright) = (
                inleft as usize,
                inright as usize,
                postleft as usize,
                postright as usize,
            );
            let root = postorder[postright];
            let in_pos = map[&root];
            let left_num = in_pos - inleft;
            Some(Rc::new(RefCell::new(TreeNode {
                val: root,
                left: dfs(
                    inorder,
                    inleft as i32,
                    in_pos as i32 - 1,
                    postorder,
                    postleft as i32,
                    (postleft + left_num) as i32 - 1,
                    map,
                ),
                right: dfs(
                    inorder,
                    in_pos as i32 + 1,
                    inright as i32,
                    postorder,
                    (postleft + left_num) as i32,
                    postright as i32 - 1,
                    map,
                ),
            })))
        }

        let mut map: HashMap<i32, usize> = inorder
            .iter() //
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, &v)| {
                acc.insert(v, i);
                acc
            });
        dfs(
            &inorder,
            0,
            inorder.len() as i32 - 1,
            &postorder,
            0,
            postorder.len() as i32 - 1,
            &mut map,
        )
    }
}
