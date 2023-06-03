// 滑动窗口最大值
// https://leetcode.cn/problems/sliding-window-maximum
// INLINE  ../../images/array/sliding_window_maximum.jpeg
// 解题思路：使用双端队列实现单调递减队列。

use std::collections::VecDeque;

pub struct Solution;

// 定义 MonotonousQueue trait，包含 push 和 pop 方法
pub trait MonotonousQueue {
    fn push(&mut self, n: i32);
    fn pop(&mut self, n: i32);
}

// 实现 MonotonousQueue trait for VecDeque<i32>
impl MonotonousQueue for VecDeque<i32> {
    // push 方法将元素插入双端队列尾部，并保证队列单调递减
    fn push(&mut self, n: i32) {
        while !self.is_empty() && *self.back().unwrap() < n {
            self.pop_back();
        }
        self.push_back(n);
    }

    // pop 方法将元素从双端队列头部弹出，如果弹出的元素是队列最大值，则队列头部元素需要更新
    fn pop(&mut self, n: i32) {
        if !self.is_empty() && *self.front().unwrap() == n {
            self.pop_front();
        }
    }
}

impl Solution {
    // max_sliding_window 方法接受一个 i32 类型的数组 nums 和一个 i32 类型的整数 k，返回一个 i32 类型的数组
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 如果 nums 数组为空或 k 等于 1，则直接返回 nums 数组
        if nums.len() == 0 || k == 1 {
            return nums;
        }

        // 定义结果数组 res 和双端队列 deque
        let mut res: Vec<i32> = Vec::new();
        let mut deque: VecDeque<i32> = VecDeque::new();

        // 遍历数组 nums
        for i in 0..nums.len() {
            // 将 nums[i] 插入双端队列尾部，保证队列单调递减
            deque.push(nums[i]);

            // 如果窗口大小为 k，说明需要弹出队列头部元素，并将队列头部元素加入结果数组 res
            if i as i32 > k - 1 {
                deque.pop(nums[i - k as usize]);

                res.push(*deque.front().unwrap());
            }
            // 如果窗口大小不足 k，说明还不能得到最大值，不需要弹出队列头部元素，但需要将队列头部元素加入结果数组 res
            else if i as i32 == k - 1 {
                res.push(*deque.front().unwrap());
            }
        }

        res
    }
}
