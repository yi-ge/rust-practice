// 通知所有员工所需的时间
// https://leetcode.cn/problems/time-needed-to-inform-all-employees
// INLINE  ../../images/tree/time_needed_to_inform_all_employees.jpeg

use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        // 定义深度优先搜索函数
        fn dfs(tree: &Vec<Vec<i32>>, inform_time: &Vec<i32>, root: i32) -> i32 {
            // 初始化最大通知时间为0
            let mut max_time = 0;
            // 遍历下属节点
            for &sub in &tree[root as usize] {
                // 计算下属节点的最大通知时间
                max_time = max(max_time, dfs(tree, inform_time, sub));
            }
            // 返回该节点通知所需时间和其下属节点的最大通知时间之和
            max_time + inform_time[root as usize]
        }

        // 初始化树结构
        let mut tree: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for i in 0..n {
            if manager[i as usize] != -1 {
                tree[manager[i as usize] as usize].push(i);
            }
        }
        // 调用深度优先搜索函数计算通知所有员工所需时间
        dfs(&tree, &inform_time, head_id)
    }
}
