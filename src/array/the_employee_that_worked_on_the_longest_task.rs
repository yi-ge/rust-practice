// 处理用时最长的那个任务的员工
// https://leetcode.cn/problems/the-employee-that-worked-on-the-longest-task
// INLINE  ../../images/array/the_employee_that_worked_on_the_longest_task.jpeg

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        // 初始化答案为第一个员工的编号
        let mut ans = logs[0][0];
        // 初始化最大用时为第一个任务的用时
        let mut max_cost = logs[0][1];
        // 遍历任务日志
        for i in 1..logs.len() {
            // 当前任务的员工编号
            let idx = logs[i][0];
            // 当前任务的用时减去上一个任务的用时
            let cost = logs[i][1] - logs[i - 1][1];
            // 比较当前任务用时和最大用时的大小关系
            match cost.cmp(&max_cost) {
                Ordering::Greater => {
                    // 如果当前任务用时大于最大用时，则更新答案为当前员工编号
                    ans = idx;
                    // 更新最大用时为当前任务用时
                    max_cost = cost;
                }
                Ordering::Equal => {
                    // 如果当前任务用时等于最大用时
                    if idx < ans {
                        // 则比较当前员工编号和答案的大小，如果小于答案则更新答案为当前员工编号
                        ans = idx;
                    }
                }
                _ => {}
            }
        }
        ans
    }
}
