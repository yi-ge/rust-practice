// 处理用时最长的那个任务的员工
// https://leetcode.cn/problems/the-employee-that-worked-on-the-longest-task
// INLINE  ../../images/array/the_employee_that_worked_on_the_longest_task.jpeg

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut ans = logs[0][0];
        let mut max_cost = logs[0][1];
        for i in 1..logs.len() {
            let idx = logs[i][0];
            let cost = logs[i][1] - logs[i - 1][1];
            match cost.cmp(&max_cost) {
                Ordering::Greater => {
                    ans = idx;
                    max_cost = cost;
                }
                Ordering::Equal => {
                    if idx < ans {
                        ans = idx;
                    }
                }
                _ => {}
            }
        }
        ans
    }
}
