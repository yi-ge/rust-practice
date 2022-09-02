// 二叉树的前序遍历
// https://leetcode.cn/problems/binary-tree-preorder-traversal/
// INLINE  ../../images/stack/binary_tree_preorder_traversal.jpeg
use std::cell::RefCell;
use std::rc::Rc;

use crate::libs::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        preorder_recursive(root, &mut result);

        result
    }
}

fn preorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        None => return,
        Some(node) => {
            result.push(node.borrow().val);
            preorder_recursive(node.borrow().left.clone(), result);
            preorder_recursive(node.borrow().right.clone(), result);
        }
    }
}
