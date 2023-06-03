// 合并两个有序链表
// https://leetcode.cn/problems/merge-two-sorted-lists/
// INLINE  ../../images/sort/merge_two_sorted_lists.jpeg

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>, // 第一个有序链表
        list2: Option<Box<ListNode>>, // 第二个有序链表
    ) -> Option<Box<ListNode>> {
        // 返回合并后的有序链表
        match (list1, list2) {
            // match语句匹配两个链表
            (Some(node1), None) => Some(node1), // 如果第二个链表为空，返回第一个链表
            (None, Some(node2)) => Some(node2), // 如果第一个链表为空，返回第二个链表
            (Some(mut node1), Some(mut node2)) => {
                // 如果两个链表都不为空
                if node1.val <= node2.val {
                    // 如果第一个链表的头结点小于等于第二个链表的头结点
                    let n = node1.next.take(); // 取出第一个链表的下一个结点
                    node1.next = Solution::merge_two_lists(n, Some(node2)); // 将第一个链表的下一个结点和第二个链表合并，赋值给第一个链表的下一个结点
                    Some(node1) // 返回合并后的第一个链表
                } else {
                    // 如果第一个链表的头结点大于第二个链表的头结点
                    let n = node2.next.take(); // 取出第二个链表的下一个结点
                    node2.next = Solution::merge_two_lists(Some(node1), n); // 将第一个链表和第二个链表的下一个结点进行合并，赋值给第二个链表的下一个结点
                    Some(node2) // 返回合并后的第二个链表
                }
            }
            _ => None, // 如果两个链表都为空，返回空
        }
    }
}
