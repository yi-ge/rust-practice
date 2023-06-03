// 合并石头的最低成本
// https://leetcode.cn/problems/minimum-cost-to-merge-stones
// INLINE  ../../images/array/minimum_cost_to_merge_stones.jpeg

pub struct Solution;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        let k = k as usize;

        // 如果无法合并成一堆，则返回-1
        if (n - 1) % (k - 1) != 0 {
            return -1;
        }

        // 计算前缀和
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        // 动态规划求解最小成本
        let mut dp = vec![vec![0; n]; n];
        for len in k..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                dp[i][j] = std::i32::MAX;
                // 枚举分割点
                for mid in (i..j).step_by(k - 1) {
                    dp[i][j] = dp[i][j].min(dp[i][mid] + dp[mid + 1][j]);
                }
                // 如果可以合并成一堆，计算成本并加入dp值中
                if (j - i) % (k - 1) == 0 {
                    dp[i][j] += prefix_sum[j + 1] - prefix_sum[i];
                }
            }
        }

        dp[0][n - 1]
    }
}
