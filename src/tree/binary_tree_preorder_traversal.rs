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
            return result; // 如果根节点为空，直接返回空结果
        }

        let mut stack = Vec::new(); // 创建一个栈，用于存储节点
        let mut node = root.clone(); // 复制根节点，作为当前节点

        while node.is_some() || !stack.is_empty() {
            // 当前节点不为空或栈不为空的情况下
            while let Some(n) = node {
                // 当前节点不为空
                result.push(n.borrow().val); // 将当前节点的值加入结果
                stack.push(n.clone()); // 当前节点入栈
                node = n.borrow().left.clone(); // 进入左子树访问
            }

            node = stack.pop(); // 栈顶元素出栈，进入右子树访问
            if let Some(n) = node {
                node = n.borrow().right.clone();
            }
        }

        result // 返回结果
    }

    // 解决方法2：递归
    pub fn preorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result; // 如果根节点为空，直接返回空结果
        }

        Self::preorder_recursive(root, &mut result); // 递归遍历

        result // 返回结果
    }

    fn preorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match root {
            None => return, // 节点为空，直接返回
            Some(node) => {
                result.push(node.borrow().val); // 将当前节点的值加入结果
                Self::preorder_recursive(node.borrow().left.clone(), result); // 递归遍历左子树
                Self::preorder_recursive(node.borrow().right.clone(), result); // 递归遍历右子树
            }
        }
    }
}
