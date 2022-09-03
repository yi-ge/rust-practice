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
        let mut node = root.clone();

        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                stack.push(n.clone());
                node = n.borrow().left.clone();
            }

            if let Some(n) = stack.pop() {
                res.push(n.borrow().val);
                node = n.borrow().right.clone();
            }
        }

        res
    }

    fn inorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => return,
            Some(node) => {
                Self::inorder_recursive(node.borrow().left.clone(), res);
                res.push(node.borrow().val);
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

        Self::inorder_recursive(root, &mut res);

        res
    }
}
