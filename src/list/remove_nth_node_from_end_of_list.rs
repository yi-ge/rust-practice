// 删除链表的倒数第 N 个结点
// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
// INLINE  ../../images/list/remove_nth_node_from_end_of_list.jpeg

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 创建一个虚拟节点dummy，值为0，next指向head
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        // 初始化慢指针slow指向dummy
        let mut slow = &mut dummy;
        // 初始化快指针fast指向slow的克隆
        let mut fast = &mut slow.clone();

        // 让fast先移动n+1步
        for _ in 1..=n + 1 {
            fast = &mut fast.as_mut().unwrap().next;
        }

        // 当fast移动到链表尾部时，slow指向的就是倒数第n+1个节点
        while fast.is_some() {
            fast = &mut fast.as_mut().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        // 删除倒数第n个节点
        let next = &slow.as_mut().unwrap().next.as_mut().unwrap().next;
        slow.as_mut().unwrap().next = next.clone();

        // 返回dummy节点的next节点
        dummy.unwrap().next
    }
}
