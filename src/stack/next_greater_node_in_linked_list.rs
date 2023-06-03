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
        // 初始化结果数组和栈
        let mut result = vec![];
        let mut s = vec![];
        let mut i = 0;
        let mut current = head;

        // 遍历链表
        while let Some(node) = current {
            // 如果栈不为空且栈顶元素小于当前节点的值，则弹出栈顶元素，将其在结果数组中对应位置的值赋为当前节点的值
            while let Some(top) = s.last() {
                if result[*top] < node.val {
                    result[*top] = node.val;
                    s.pop();
                } else {
                    break;
                }
            }

            // 将当前节点在结果数组中的位置压入栈中，并将节点的值加入结果数组
            s.push(i);
            result.push(node.val);
            current = node.next;
            i += 1;
        }

        // 如果栈不为空，则说明栈中剩余的元素没有下一个更大的节点，将它们在结果数组中对应位置的值赋为0
        while let Some(top) = s.pop() {
            result[top] = 0;
        }

        result
    }
}
