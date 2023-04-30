// 通知所有员工所需的时间
// https://leetcode.cn/problems/time-needed-to-inform-all-employees
// INLINE  ../../images/tree/time_needed_to_inform_all_employees.jpeg

use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        fn dfs(tree: &Vec<Vec<i32>>, inform_time: &Vec<i32>, root: i32) -> i32 {
            let mut max_time = 0;
            for &sub in &tree[root as usize] {
                max_time = max(max_time, dfs(tree, inform_time, sub));
            }
            max_time + inform_time[root as usize]
        }

        let mut tree: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for i in 0..n {
            if manager[i as usize] != -1 {
                tree[manager[i as usize] as usize].push(i);
            }
        }
        dfs(&tree, &inform_time, head_id)
    }
}
