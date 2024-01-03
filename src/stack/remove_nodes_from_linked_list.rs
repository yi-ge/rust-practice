// 从链表中移除节点
// https://leetcode.cn/problems/remove-nodes-from-linked-list
// INLINE  ../../images/stack/remove_nodes_from_linked_list.jpeg

use crate::libs::list_node::ListNode;

pub struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut head = head;
        let next = head.as_mut().unwrap().next.take();
        head.as_mut().unwrap().next = Solution::remove_nodes(next);
        if head.as_ref().unwrap().next.is_some()
            && head.as_ref().unwrap().val < head.as_ref().unwrap().next.as_ref().unwrap().val
        {
            return head.as_mut().unwrap().next.take();
        } else {
            return head;
        }
    }
}
