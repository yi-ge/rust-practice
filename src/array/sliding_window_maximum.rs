// 滑动窗口最大值
// https://leetcode.cn/problems/sliding-window-maximum
// INLINE  ../../images/array/sliding_window_maximum.jpeg
// 解题思路：单调队列

use std::collections::VecDeque;

pub struct Solution;

pub trait MonotonousQueue {
    fn push(&mut self, n: i32);
    fn pop(&mut self, n: i32);
}

impl MonotonousQueue for VecDeque<i32> {
    fn push(&mut self, n: i32) {
        while !self.is_empty() && *self.back().unwrap() < n {
            self.pop_back();
        }
        self.push_back(n);
    }

    fn pop(&mut self, n: i32) {
        if !self.is_empty() && *self.front().unwrap() == n {
            self.pop_front();
        }
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 || k == 1 {
            return nums;
        }

        let mut res: Vec<i32> = Vec::new();
        let mut deque: VecDeque<i32> = VecDeque::new();

        for i in 0..nums.len() {
            deque.push(nums[i]);

            if i as i32 > k - 1 {
                deque.pop(nums[i - k as usize]);

                res.push(*deque.front().unwrap());
            } else if i as i32 == k - 1 {
                res.push(*deque.front().unwrap());
            }
        }

        res
    }
}
