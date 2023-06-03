// 二叉搜索子树的最大键值和
// https://leetcode.cn/problems/maximum-sum-bst-in-binary-tree
// INLINE  ../../images/other/maximum_sum_bst_in_binary_tree.jpeg

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
    // dfs函数，返回一个元组，包含四个元素
    // 第一个元素表示以此节点为根的子树是否为BST
    // 第二个元素表示以此节点为根的子树的最大键值和
    // 第三个元素表示以此节点为根的子树的最小值
    // 第四个元素表示以此节点为根的子树的最大值
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> (bool, i32, i32, i32) {
        // 如果当前节点不为空
        if let Some(node) = root {
            let node = node.borrow();
            let (left_bst, left_sum, left_min, left_max) = Self::dfs(&node.left, max_sum);
            let (right_bst, right_sum, right_min, right_max) = Self::dfs(&node.right, max_sum);

            // 如果左右子树都是BST，并且左子树的最大值小于当前节点的值，右子树的最小值大于当前节点的值
            if left_bst && right_bst && left_max < node.val && node.val < right_min {
                // 计算当前子树的键值和
                let sum = left_sum + right_sum + node.val;
                // 更新最大键值和
                *max_sum = (*max_sum).max(sum);
                // 返回当前子树为BST，最大键值和为sum，最小值为left_min和当前节点值中的较小值，最大值为right_max和当前节点值中的较大值
                return (true, sum, left_min.min(node.val), right_max.max(node.val));
            }
        } else {
            // 当前节点为空，返回一个默认元组，表示此处的子树不是BST
            return (true, 0, i32::MAX, i32::MIN);
        }

        // 如果上面的条件都不成立，返回一个默认元组，表示此处的子树不是BST
        (false, -1, i32::MAX, i32::MIN)
    }

    // 计算二叉树中BST的最大键值和
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = 0;
        Self::dfs(&root, &mut max_sum);
        max_sum
    }
}
