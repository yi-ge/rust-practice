// 从链表中删去总和值为零的连续节点
// https://leetcode.cn/problems/remove-zero-sum-consecutive-nodes-from-linked-list
// INLINE  ../../images/list/remove_zero_sum_consecutive_nodes_from_linked_list.jpeg

use std::collections::HashMap;

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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 如果链表头为空，则直接返回链表头
        if head.is_none() {
            return head;
        }
        // 创建一个新的链表头，值为0，next为原链表头
        let p_head = Some(Box::new(ListNode { val: 0, next: head }));
        // 创建一个HashMap，用来存储链表节点的和，key为和，value为链表节点
        let mut hash_map = HashMap::new();
        // 复制链表头，用来遍历链表
        let mut head_clone = p_head.clone();
        // 创建一个链表引用，用来遍历链表
        let mut head_refer = &p_head;

        // 创建一个和的变量，用来存储当前遍历节点的和
        let mut sum = 0;
        // 遍历链表
        while head_refer.is_some() {
            // 计算当前节点的和
            sum = sum + &head_refer.as_ref()?.val;
            // 将当前节点的和和当前节点的引用存储到HashMap中
            hash_map.insert(sum, head_refer.clone());
            // 遍历下一个节点
            head_refer = &head_refer.as_ref()?.next;
        }
        // 将链表引用指向复制的链表头
        let mut head_refer = &mut head_clone;
        // 将和的变量置0
        sum = 0;
        // 遍历链表
        while head_refer.is_some() {
            // 计算当前节点的和
            sum += &head_refer.as_mut()?.val;
            // 通过和在HashMap中找到和为当前节点和的节点
            let node = hash_map.get_mut(&sum)?;
            // 将当前节点的next指向和为当前节点和的节点的next
            head_refer.as_mut()?.next = node.as_mut()?.next.take();
            // 遍历下一个节点
            head_refer = &mut head_refer.as_mut()?.next;
        }
        // 返回链表头
        head_clone?.next
    }
}
