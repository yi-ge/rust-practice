// 从二叉搜索树到更大和树
// https://leetcode.cn/problems/binary-search-tree-to-greater-sum-tree
// INLINE  ../../images/tree/binary_search_tree_to_greater_sum_tree.jpeg

use std::{cell::RefCell, rc::Rc};

use crate::libs::tree_node::TreeNode;

pub struct Solution;

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        let cloned_root = root.clone(); // 克隆root
        Solution::bst_to_gst_helper(cloned_root, &mut sum);
        root
    }

    fn bst_to_gst_helper(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = root {
            Solution::bst_to_gst_helper(node.borrow().right.clone(), sum);
            *sum += node.borrow().val;
            node.borrow_mut().val = *sum;
            Solution::bst_to_gst_helper(node.borrow().left.clone(), sum);
        }
    }
}
