// 二叉树的前序遍历
// https://leetcode.cn/problems/binary-tree-preorder-traversal/
// INLINE  ../../images/tree/binary_tree_preorder_traversal.jpeg
use std::cell::RefCell;
use std::rc::Rc;

use crate::libs::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    // 解决方法1：循环
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut stack = Vec::new();
        let mut node = root.clone();

        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                result.push(n.borrow().val);
                stack.push(n.clone()); // 当前节点入栈
                node = n.borrow().left.clone(); // 进入左子树访问
            }

            node = stack.pop(); // 栈顶元素出栈，进入右子树访问
            if let Some(n) = node {
                node = n.borrow().right.clone();
            }
        }

        result
    }

    // 解决方法2：递归
    pub fn preorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        Self::preorder_recursive(root, &mut result);

        result
    }

    fn preorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match root {
            None => return,
            Some(node) => {
                result.push(node.borrow().val);
                Self::preorder_recursive(node.borrow().left.clone(), result);
                Self::preorder_recursive(node.borrow().right.clone(), result);
            }
        }
    }
}
