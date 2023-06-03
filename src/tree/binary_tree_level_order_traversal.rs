// 二叉树的层序遍历
// https://leetcode.cn/problems/binary-tree-level-order-traversal
// INLINE  ../../images/tree/binary_tree_level_order_traversal.jpeg

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
use std::collections::VecDeque;
use std::rc::Rc;

use crate::libs::tree_node::TreeNode;

impl Solution {
    // 二叉树的层序遍历函数
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // 定义返回结果的二维数组
        let mut res = vec![];
        // 如果根节点为空，直接返回结果
        if root.is_none() {
            return res;
        }

        // 使用双端队列存储节点
        let mut deque = VecDeque::new();
        deque.push_back(root);

        // 遍历二叉树
        while !deque.is_empty() {
            // 定义当前层的数组
            let mut current_level = vec![];
            // 遍历当前层的所有节点
            for _ in 0..deque.len() {
                // 弹出队首节点
                if let Some(Some(n)) = deque.pop_front() {
                    // 将节点的值存储到当前层的数组中
                    current_level.push(n.borrow().val);

                    // 如果有左子节点，将左子节点加入队列
                    if n.borrow().left.is_some() {
                        deque.push_back(n.borrow().left.clone());
                    }

                    // 如果有右子节点，将右子节点加入队列
                    if n.borrow().right.is_some() {
                        deque.push_back(n.borrow().right.clone());
                    }
                }
            }
            // 将当前层的数组存储到返回结果中
            res.push(current_level);
        }

        // 返回结果
        res
    }
}
