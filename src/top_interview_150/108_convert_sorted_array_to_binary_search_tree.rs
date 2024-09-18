use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                None
            } else {
                let idx = nums.len() / 2;
                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[idx],
                    left: dfs(&nums[..idx]),
                    right: dfs(&nums[idx + 1..]),
                })))
            }
        }
        dfs(&nums)
    }
}
