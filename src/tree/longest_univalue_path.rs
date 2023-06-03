// 最长同值路径
// https://leetcode.cn/problems/longest-univalue-path
// INLINE  ../../images/tree/longest_univalue_path.jpeg
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

use crate::libs::tree_node::TreeNode;

impl Solution {
    // 定义一个 DFS 函数，返回以当前节点为根节点的最长同值路径的长度，并更新答案 max
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32, parent_val: i32) -> i32 {
        // 如果当前节点为空，返回 0
        if let Some(node) = root {
            // 递归左右子树，并得到左右子树的最长同值路径长度
            let left = Self::dfs(node.borrow().left.as_ref(), max, node.borrow().val);
            let right = Self::dfs(node.borrow().right.as_ref(), max, node.borrow().val);

            // 更新答案 max
            *max = (*max).max(left + right);

            // 如果当前节点的值等于父节点的值，返回左右子树中最长同值路径长度的较大值加 1
            if node.borrow().val == parent_val {
                return left.max(right) + 1;
            }
        }

        // 否则返回 0
        0
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 初始化答案为 0
        let mut ans = 0;
        // 调用 DFS 函数，更新答案 max
        Self::dfs(root.as_ref(), &mut ans, -1);
        ans
    }
}
