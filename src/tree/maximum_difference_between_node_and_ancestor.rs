// 节点与其祖先之间的最大差值
// https://leetcode.cn/problems/maximum-difference-between-node-and-ancestor
// INLINE  ../../images/tree/maximum_difference_between_node_and_ancestor.jpeg

pub struct Solution;

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

use crate::libs::tree_node::TreeNode;

impl Solution {
    // 递归函数，返回当前子树中节点的最小值和最大值
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_diff: &mut i32) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let (mut min, mut max) = (node.val, node.val);
            // 如果有左子树，递归左子树
            if let Some(left) = &node.left {
                let (left_min, left_max) = Self::dfs(Some(Rc::clone(left)), max_diff);
                min = min.min(left_min);
                max = max.max(left_max);
            }
            // 如果有右子树，递归右子树
            if let Some(right) = &node.right {
                let (right_min, right_max) = Self::dfs(Some(Rc::clone(right)), max_diff);
                min = min.min(right_min);
                max = max.max(right_max);
            }
            // 更新最大差值
            *max_diff = (*max_diff)
                .max((node.val - min).abs())
                .max((node.val - max).abs());
            (min, max)
        } else {
            // 如果节点为 None，返回 (0, 0)
            #[cfg_attr(tarpaulin, skip)]
            (0, 0)
        }
    }
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diff = 0;
        // 调用递归函数
        Self::dfs(root, &mut max_diff);
        max_diff
    }
}
