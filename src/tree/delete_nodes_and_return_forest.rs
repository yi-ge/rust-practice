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
        // 将要删除的节点集合转换为哈希集合
        let to_delete_set: HashSet<_> = to_delete.into_iter().collect();
        let mut res = Vec::new();
        // 递归遍历二叉树
        let root = Self::dfs(root, &to_delete_set, &mut res);
        // 如果根节点未被删除，则将其添加到结果中
        if root.is_some() {
            res.push(root);
        }
        res
    }

    // 递归遍历二叉树
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete_set: &HashSet<i32>,
        res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut inner_node = node.borrow_mut();
            // 遍历左子树
            inner_node.left = Self::dfs(inner_node.left.clone(), to_delete_set, res);
            // 遍历右子树
            inner_node.right = Self::dfs(inner_node.right.clone(), to_delete_set, res);

            // 如果当前节点需要被删除
            if to_delete_set.contains(&inner_node.val) {
                // 如果左子树不为空，则将其添加到结果中
                if inner_node.left.is_some() {
                    res.push(inner_node.left.clone());
                }
                // 如果右子树不为空，则将其添加到结果中
                if inner_node.right.is_some() {
                    res.push(inner_node.right.clone());
                }
                // 返回空节点
                return None;
            }
        }
        // 返回当前节点
        root
    }
}
