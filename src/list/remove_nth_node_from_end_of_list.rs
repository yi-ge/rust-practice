// 删除链表的倒数第 N 个结点
// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
// INLINE  ../../images/list/remove_nth_node_from_end_of_list.jpeg

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut dummy;
        let mut fast = &mut slow.clone();

        for _ in 1..=n + 1 {
            fast = &mut fast.as_mut().unwrap().next;
        }

        while fast.is_some() {
            fast = &mut fast.as_mut().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        let next = &slow.as_mut().unwrap().next.as_mut().unwrap().next;
        slow.as_mut().unwrap().next = next.clone();

        dummy.unwrap().next
    }
}
