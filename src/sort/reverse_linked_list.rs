// 反转链表
// https://leetcode.cn/problems/reverse-linked-list/
// INLINE  ../../images/sort/reverse_linked_list.jpeg

use crate::libs::list_node::ListNode;

pub struct Solution {}

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut prev = None;
      let mut curr = head;

      while let Some(mut curr_node) = curr.take() {
        let next_tmp = curr_node.next.take();
        curr_node.next = prev.take();
        prev = Some(curr_node);
        curr = next_tmp;
      }

      prev
    }
}