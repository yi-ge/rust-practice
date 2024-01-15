// 删除排序链表中的重复元素 II
// https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii
// INLINE  ../../images/list/remove_duplicates_from_sorted_list_ii.jpeg

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy_head;
        let mut deleted_val = None;
        while cur.next.is_some() {
            if Some(cur.next.as_ref().unwrap().val) == deleted_val
                || cur.next.as_ref().unwrap().next.is_some()
                    && cur.next.as_ref().unwrap().val
                        == cur.next.as_ref().unwrap().next.as_ref().unwrap().val
            {
                deleted_val = Some(cur.next.as_ref().unwrap().val);
                cur.next = cur.next.take().unwrap().next;
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy_head.next
    }
}
