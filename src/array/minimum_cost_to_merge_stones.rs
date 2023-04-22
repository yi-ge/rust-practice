// 合并石头的最低成本
// https://leetcode.cn/problems/minimum-cost-to-merge-stones
// INLINE  ../../images/array/minimum_cost_to_merge_stones.jpeg

pub struct Solution;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        let k = k as usize;

        if (n - 1) % (k - 1) != 0 {
            return -1;
        }

        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        let mut dp = vec![vec![0; n]; n];
        for len in k..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                dp[i][j] = std::i32::MAX;
                for mid in (i..j).step_by(k - 1) {
                    dp[i][j] = dp[i][j].min(dp[i][mid] + dp[mid + 1][j]);
                }
                if (j - i) % (k - 1) == 0 {
                    dp[i][j] += prefix_sum[j + 1] - prefix_sum[i];
                }
            }
        }

        dp[0][n - 1]
    }
}
