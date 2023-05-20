// 二叉搜索子树的最大键值和
// https://leetcode.cn/problems/maximum-sum-bst-in-binary-tree
// INLINE  ../../images/other/maximum_sum_bst_in_binary_tree.jpeg

pub struct Solution;
use crate::libs::tree_node::TreeNode;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> (bool, i32, i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let (left_bst, left_sum, left_min, left_max) = Self::dfs(&node.left, max_sum);
            let (right_bst, right_sum, right_min, right_max) = Self::dfs(&node.right, max_sum);

            if left_bst && right_bst && left_max < node.val && node.val < right_min {
                let sum = left_sum + right_sum + node.val;
                *max_sum = (*max_sum).max(sum);
                return (true, sum, left_min.min(node.val), right_max.max(node.val));
            }
        } else {
            return (true, 0, i32::MAX, i32::MIN);
        }

        (false, -1, i32::MAX, i32::MIN)
    }

    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = 0;
        Self::dfs(&root, &mut max_sum);
        max_sum
    }
}
