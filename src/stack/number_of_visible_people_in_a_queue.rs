// 队列中可以看到的人数
// https://leetcode.cn/problems/number-of-visible-people-in-a-queue
// INLINE  ../../images/stack/number_of_visible_people_in_a_queue.jpeg

pub struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut res = vec![0; n];
        let mut stack = Vec::new();

        for i in (0..n).rev() {
            let h = heights[i];
            while !stack.is_empty() && *stack.last().unwrap() < h {
                stack.pop();
                res[i] += 1;
            }
            if !stack.is_empty() {
                res[i] += 1;
            }
            stack.push(h);
        }
        res
    }
}
