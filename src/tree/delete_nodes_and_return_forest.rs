// 删点成林
// https://leetcode.cn/problems/delete-nodes-and-return-forest
// INLINE  ../../images/tree/delete_nodes_and_return_forest.jpeg

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
use std::collections::HashSet;
use std::rc::Rc;

use crate::libs::tree_node::TreeNode;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let to_delete_set: HashSet<_> = to_delete.into_iter().collect();
        let mut res = Vec::new();
        let root = Self::dfs(root, &to_delete_set, &mut res);
        if root.is_some() {
            res.push(root);
        }
        res
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete_set: &HashSet<i32>,
        res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut inner_node = node.borrow_mut();
            inner_node.left = Self::dfs(inner_node.left.clone(), to_delete_set, res);
            inner_node.right = Self::dfs(inner_node.right.clone(), to_delete_set, res);

            if to_delete_set.contains(&inner_node.val) {
                if inner_node.left.is_some() {
                    res.push(inner_node.left.clone());
                }
                if inner_node.right.is_some() {
                    res.push(inner_node.right.clone());
                }
                return None;
            }
        }
        root
    }
}
