// 二叉树的中序遍历
// https://leetcode.cn/problems/binary-tree-inorder-traversal
// INLINE  ../../images/tree/binary_tree_inorder_traversal.jpeg

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
    // 解决方法1： 循环法
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        let mut stack = vec![]; // 用栈保存需要处理的结点
        let mut node = root.clone(); // 复制一份根结点，避免修改原结点

        // 当结点不为空或栈不为空时，进行循环
        while node.is_some() || !stack.is_empty() {
            // 将结点的所有左子结点都压入栈中
            while let Some(n) = node {
                stack.push(n.clone()); // 将结点复制并压入栈中
                node = n.borrow().left.clone(); // 处理左子结点
            }

            // 如果栈不为空，弹出栈顶结点，并处理右子结点
            if let Some(n) = stack.pop() {
                res.push(n.borrow().val); // 处理栈顶结点
                node = n.borrow().right.clone(); // 处理右子结点
            }
        }

        res // 返回结果
    }

    // 递归函数，用于中序遍历二叉树
    fn inorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => return, // 如果结点为空，直接返回
            Some(node) => {
                // 先处理左子结点
                Self::inorder_recursive(node.borrow().left.clone(), res);
                // 再处理根结点
                res.push(node.borrow().val);
                // 最后处理右子结点
                Self::inorder_recursive(node.borrow().right.clone(), res);
            }
        }
    }

    // 解决方法2：递归法
    pub fn inorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        Self::inorder_recursive(root, &mut res); // 调用递归函数

        res // 返回结果
    }
}
