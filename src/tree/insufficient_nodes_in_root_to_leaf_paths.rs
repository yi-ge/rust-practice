// 根到叶路径上的不足节点
// https://leetcode.cn/problems/insufficient-nodes-in-root-to-leaf-paths
// INLINE  ../../images/tree/insufficient_nodes_in_root_to_leaf_paths.jpeg

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
    fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, sum: i32, limit: i32) -> bool {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let new_sum = sum + node.val;

            if node.left.is_none() && node.right.is_none() {
                return new_sum < limit;
            }

            let left_remove = Self::dfs(&mut node.left, new_sum, limit);
            let right_remove = Self::dfs(&mut node.right, new_sum, limit);

            if left_remove {
                node.left = None;
            }
            if right_remove {
                node.right = None;
            }

            return node.left.is_none() && node.right.is_none();
        } else {
            false
        }
    }

    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;

        if Self::dfs(&mut root, 0, limit) {
            None
        } else {
            root
        }
    }
}
