// 验证栈序列
// https://leetcode.cn/problems/validate-stack-sequences
// INLINE  ../../images/heap/validate_stack_sequences.jpeg
// 解题思路：模拟栈操作

use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut queue = VecDeque::new();
        let mut i = 0;
        for push in pushed {
            queue.push_back(push);

            while !queue.is_empty() && **queue.back().as_ref().unwrap() == popped[i] {
                queue.pop_back();
                i += 1;
            }
        }

        queue.is_empty()
    }
}
