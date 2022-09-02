// 二叉树的后序遍历
// https://leetcode.cn/problems/binary-tree-postorder-traversal
// INLINE  ../../images/tree/binary_tree_postorder_traversal.jpeg

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
use std::rc::Rc;

use crate::libs::tree_node::TreeNode;
impl Solution {
    // 解决方法1： 循环
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        stack1.push(root.clone());

        while let Some(Some(node)) = stack1.pop() {
            if node.borrow().left.is_some() {
                stack1.push(node.borrow().left.clone());
            }

            if node.borrow().right.is_some() {
                stack1.push(node.borrow().right.clone());
            }

            stack2.push(Some(node));
        }

        while let Some(Some(node)) = stack2.pop() {
            res.push(node.borrow().val);
        }

        res
    }

    pub fn postorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => (),
            Some(node) => {
                Self::postorder_recursive(node.borrow().left.clone(), res);
                Self::postorder_recursive(node.borrow().right.clone(), res);
                res.push(node.borrow().val);
            }
        }
    }

    // 解决方法2：递归
    pub fn postorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        Self::postorder_recursive(root, &mut res);

        res
    }
}
