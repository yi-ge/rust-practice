// 合并两个有序链表
// https://leetcode.cn/problems/merge-two-sorted-lists/
// INLINE  ../../images/sort/merge_two_sorted_lists.jpeg

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val <= node2.val {
                    let n = node1.next.take();
                    node1.next = Solution::merge_two_lists(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Solution::merge_two_lists(Some(node1), n);
                    Some(node2)
                }
            }
            _ => None,
        }
    }
}
