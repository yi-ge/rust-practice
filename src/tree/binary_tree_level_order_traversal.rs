// 二叉树的层序遍历
// https://leetcode.cn/problems/binary-tree-level-order-traversal
// INLINE  ../../images/tree/binary_tree_level_order_traversal.jpeg

pub struct Solution {}

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
use std::collections::VecDeque;
use std::rc::Rc;

use crate::libs::tree_node::TreeNode;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        let mut deque = VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            let mut current_level = vec![];
            for _ in 0..deque.len() {
                if let Some(Some(n)) = deque.pop_front() {
                    current_level.push(n.borrow().val);

                    if n.borrow().left.is_some() {
                        deque.push_back(n.borrow().left.clone());
                    }

                    if n.borrow().right.is_some() {
                        deque.push_back(n.borrow().right.clone());
                    }
                }
            }
            res.push(current_level);
        }

        res
    }
}
