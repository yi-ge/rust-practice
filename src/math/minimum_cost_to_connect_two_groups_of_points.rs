// 连通两组点的最小成本
// https://leetcode.cn/problems/minimum-cost-to-connect-two-groups-of-points
// INLINE  ../../images/math/minimum_cost_to_connect_two_groups_of_points.jpeg

use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        // 获取cost的行数和列数
        let size1 = cost.len();
        let size2 = cost[0].len();
        // 计算可能的状态数量，等于2的size2次方
        let m = 1 << size2;
        // 初始化两个动态规划数组，dp数组存储当前的最小成本，dp1数组用于计算下一行的最小成本
        let mut dp: Vec<i32> = vec![i32::MAX / 2; m];
        let mut dp1: Vec<i32> = vec![0; m];
        // 第一个状态的成本为0
        dp[0] = 0;
        // 遍历每一行（即每一个点）
        for i in 1..=size1 {
            // 对于每一行，遍历所有可能的状态
            for s in 0..m {
                // 初始化dp1[s]为最大值
                dp1[s] = i32::MAX / 2;
                // 遍历每一点
                for k in 0..size2 {
                    // 如果第k点未被选中（即第k位为0），则跳过此次循环
                    if (s & (1 << k)) == 0 {
                        continue;
                    }
                    // 更新dp1[s]，考虑通过k点连接当前行和前一行的三种可能性，取最小值
                    dp1[s] = min(dp1[s], dp1[s ^ (1 << k)] + cost[i - 1][k]);
                    dp1[s] = min(dp1[s], dp[s] + cost[i - 1][k]);
                    dp1[s] = min(dp1[s], dp[s ^ (1 << k)] + cost[i - 1][k]);
                }
            }
            // 更新dp数组，将dp1的值复制到dp
            dp = dp1.clone();
        }
        // 返回连接所有点的最小成本，即dp[m - 1]
        return dp[m - 1];
    }
}
