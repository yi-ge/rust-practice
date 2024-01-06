// 在链表中插入最大公约数
// https://leetcode.cn/problems/insert-greatest-common-divisors-in-linked-list
// INLINE  ../../images/list/insert_greatest_common_divisors_in_linked_list.jpeg

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
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        // 如果 a 或 b 是负数，那么取绝对值
        a = a.abs();
        b = b.abs();
        // 如果 a 或 b 是 0，那么返回另一个数
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        a
    }

    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut cur = &mut head;
        while cur.as_ref().unwrap().next.is_some() {
            let x = cur.as_mut().unwrap();
            let next = x.next.take();
            x.next = Some(Box::new(ListNode {
                val: Self::gcd(x.val, next.as_ref().unwrap().val),
                next,
            }));
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }
}
