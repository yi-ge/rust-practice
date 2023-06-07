// 老鼠和奶酪
// https://leetcode.cn/problems/mice-and-cheese
// INLINE  ../../images/sort/mice_and_cheese.jpeg

pub struct Solution;

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        // 总奖励
        let mut total_reward = 0;
        let mut reward_differences = Vec::with_capacity(reward1.len());

        for i in 0..reward1.len() {
            total_reward += reward2[i];
            reward_differences.push(reward1[i] - reward2[i]);
        }

        // 按照差值降序排列
        reward_differences.sort_unstable_by(|a, b| b.cmp(a));
        for i in 0..k {
            total_reward += reward_differences[i as usize];
        }

        total_reward
    }
}
