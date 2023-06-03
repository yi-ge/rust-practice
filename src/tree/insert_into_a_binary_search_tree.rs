// 二叉搜索树中的插入操作
// https://leetcode.cn/problems/insert-into-a-binary-search-tree
// INLINE  ../../images/tree/insert_into_a_binary_search_tree.jpeg

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
use std::rc::Rc;

use crate::libs::tree_node::TreeNode;
impl Solution {
    // 插入节点的方法，递归实现
    pub fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(node) = root {
            let mut cur = node.borrow_mut(); // 获取当前节点的可变引用
            let target = if val > cur.val {
                // 判断要插入的值与当前节点的大小关系
                &mut cur.right // 如果大于当前节点，则插入到当前节点的右子树
            } else {
                &mut cur.left // 否则插入到当前节点的左子树
            };

            if target.is_some() {
                // 如果子树不为空，则递归继续插入
                return Self::insert(target, val);
            }

            // 如果子树为空，则插入新节点
            *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }

    // 插入节点的入口方法，返回插入后的根节点
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val)))); // 如果根节点为空，则直接插入
        }

        Self::insert(&root, val); // 否则递归插入

        root // 返回根节点
    }
}
