// 根到叶路径上的不足节点
// https://leetcode.cn/problems/insufficient-nodes-in-root-to-leaf-paths
// INLINE  ../../images/tree/insufficient_nodes_in_root_to_leaf_paths.jpeg

pub struct Solution;
use crate::libs::tree_node::TreeNode;

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

impl Solution {
    // 深度优先搜索
    fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, sum: i32, limit: i32) -> bool {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let new_sum = sum + node.val;

            // 如果当前节点是叶子节点，则比较路径上的和与限制的大小
            if node.left.is_none() && node.right.is_none() {
                return new_sum < limit;
            }

            // 递归处理左右子树
            let left_remove = Self::dfs(&mut node.left, new_sum, limit);
            let right_remove = Self::dfs(&mut node.right, new_sum, limit);

            // 如果左子树需要删除，则将左子树置为None
            if left_remove {
                node.left = None;
            }
            // 如果右子树需要删除，则将右子树置为None
            if right_remove {
                node.right = None;
            }

            // 如果左右子树都已经被删除，则当前节点也需要被删除
            return node.left.is_none() && node.right.is_none();
        } else {
            false
        }
    }

    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;

        // 如果根节点需要被删除，则返回None
        if Self::dfs(&mut root, 0, limit) {
            None
        } else {
            root
        }
    }
}
