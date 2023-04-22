// 链表中的下一个更大节点
// https://leetcode.cn/problems/next-greater-node-in-linked-list
// INLINE  ../../images/stack/next_greater_node_in_linked_list.jpeg

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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut s = vec![];
        let mut i = 0;
        let mut current = head;

        while let Some(node) = current {
            while let Some(top) = s.last() {
                if result[*top] < node.val {
                    result[*top] = node.val;
                    s.pop();
                } else {
                    break;
                }
            }

            s.push(i);
            result.push(node.val);
            current = node.next;
            i += 1;
        }

        while let Some(top) = s.pop() {
            result[top] = 0;
        }

        result
    }
}
