// 链表的中间结点
// https://leetcode.cn/problems/middle-of-the-linked-list/
// INLINE  ../../images/list/middle_of_the_linked_list.jpeg
// 解题思路：快慢指针

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 定义快慢指针
        let mut fast = &head;
        let mut slow = &head;

        // 快指针每次移动两步，慢指针每次移动一步
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }

        // 返回慢指针所指向的结点
        slow.clone()
    }
}
