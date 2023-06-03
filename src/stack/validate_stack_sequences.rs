// 验证栈序列
// https://leetcode.cn/problems/validate-stack-sequences
// INLINE  ../../images/heap/validate_stack_sequences.jpeg
// 解题思路：模拟栈操作

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        // 创建一个双向队列
        let mut queue = VecDeque::new();
        // 初始化下标i
        let mut i = 0;
        // 遍历pushed数组中的元素
        for push in pushed {
            // 将元素添加到队列的末尾
            queue.push_back(push);

            // 当队列不为空且队列中的末尾元素等于popped中下标为i的元素时
            while !queue.is_empty() && **queue.back().as_ref().unwrap() == popped[i] {
                // 从队列的末尾弹出元素
                queue.pop_back();
                // 将i加1
                i += 1;
            }
        }

        // 判断队列是否为空
        queue.is_empty()
    }
}
