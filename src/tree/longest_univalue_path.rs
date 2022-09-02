// 最长同值路径
// https://leetcode.cn/problems/longest-univalue-path
// INLINE  ../../images/tree/longest_univalue_path.jpeg
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

use crate::libs::tree_node::TreeNode;
impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32, parent_val: i32) -> i32 {
        if let Some(node) = root {
            let left = Self::dfs(node.borrow().left.as_ref(), max, node.borrow().val);
            let right = Self::dfs(node.borrow().right.as_ref(), max, node.borrow().val);

            *max = (*max).max(left + right);

            if node.borrow().val == parent_val {
                return left.max(right) + 1;
            }
        }

        0
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(root.as_ref(), &mut ans, -1);
        ans
    }
}
