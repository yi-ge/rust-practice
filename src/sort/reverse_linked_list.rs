// 反转链表
// https://leetcode.cn/problems/reverse-linked-list/
// INLINE  ../../images/sort/reverse_linked_list.jpeg

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None; // 定义一个空节点prev
        let mut curr = head; // 定义一个可变节点curr，值为head

        while let Some(mut curr_node) = curr.take() {
            // 当curr_node存在时，执行以下操作
            let next_tmp = curr_node.next.take(); // 将curr_node的下一个节点赋值给next_tmp
            curr_node.next = prev.take(); // 将prev的值赋给curr_node的下一个节点
            prev = Some(curr_node); // 将curr_node的值赋给prev
            curr = next_tmp; // 将next_tmp赋值给curr
        }

        prev // 返回prev
    }
}
