// 使用最小花费爬楼梯
// https://leetcode.cn/problems/min-cost-climbing-stairs
// INLINE  ../../images/array/min_cost_climbing_stairs.jpeg

pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 1];
        for i in 2..=cost.len() {
            dp[i] = std::cmp::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
        }
        dp[cost.len()] as i32
    }
}
