// 二叉搜索树中的插入操作
// https://leetcode.cn/problems/insert-into-a-binary-search-tree
// INLINE  ../../images/tree/insert_into_a_binary_search_tree.jpeg

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
    pub fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(node) = root {
            let mut cur = node.borrow_mut();
            let target = if val > cur.val {
                &mut cur.right
            } else {
                &mut cur.left
            };

            if target.is_some() {
                return Self::insert(target, val);
            }

            *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        Self::insert(&root, val);

        root
    }
}
