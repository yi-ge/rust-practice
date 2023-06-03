// 二叉树的后序遍历
// https://leetcode.cn/problems/binary-tree-postorder-traversal
// INLINE  ../../images/tree/binary_tree_postorder_traversal.jpeg

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
    // 解决方法1： 循环
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![]; // 存储结果的数组
        if root.is_none() {
            // 如果根节点为空，则直接返回结果数组
            return res;
        }

        let mut stack1 = Vec::new(); // 存储节点的栈
        let mut stack2 = Vec::new(); // 存储节点值的栈
        stack1.push(root.clone()); // 将根节点加入节点栈中

        while let Some(Some(node)) = stack1.pop() {
            // 循环直到节点栈为空
            if node.borrow().left.is_some() {
                // 如果节点有左子树，则将其加入节点栈中
                stack1.push(node.borrow().left.clone());
            }

            if node.borrow().right.is_some() {
                // 如果节点有右子树，则将其加入节点栈中
                stack1.push(node.borrow().right.clone());
            }

            stack2.push(Some(node)); // 将节点加入节点值栈中
        }

        while let Some(Some(node)) = stack2.pop() {
            // 循环直到节点值栈为空
            res.push(node.borrow().val); // 将节点值加入结果数组中
        }

        res // 返回结果数组
    }

    // 递归函数，用于后序遍历
    pub fn postorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => (), // 如果节点为空，则直接返回
            Some(node) => {
                Self::postorder_recursive(node.borrow().left.clone(), res); // 遍历左子树
                Self::postorder_recursive(node.borrow().right.clone(), res); // 遍历右子树
                res.push(node.borrow().val); // 将节点值加入结果数组中
            }
        }
    }

    // 解决方法2：递归
    pub fn postorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![]; // 存储结果的数组
        if root.is_none() {
            // 如果根节点为空，则直接返回结果数组
            return res;
        }

        Self::postorder_recursive(root, &mut res); // 调用递归函数遍历树

        res // 返回结果数组
    }
}
