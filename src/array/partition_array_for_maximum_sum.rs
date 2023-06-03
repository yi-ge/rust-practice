// 分隔数组以得到最大和
// https://leetcode.cn/problems/partition-array-for-maximum-sum
// INLINE  ../../images/array/partition_array_for_maximum_sum.jpeg

pub struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![0; n + 1]; // 动态规划表，用于存储最大和

        // 遍历数组
        for i in 1..=n {
            let mut max_val_in_partition = arr[i - 1]; // 初始化当前分割中的最大值

            // 从当前位置（i）反向迭代，最多迭代k个元素
            let mut j = i.saturating_sub(1);
            while j + k as usize >= i {
                // 通过比较当前值和前一个分割的和来更新最大和
                dp[i] = std::cmp::max(dp[i], dp[j] + max_val_in_partition * (i - j) as i32);

                // 如果分割中仍有元素，则更新max_val_in_partition
                if j > 0 {
                    max_val_in_partition = std::cmp::max(max_val_in_partition, arr[j - 1]);
                    j -= 1;
                } else {
                    break;
                }
            }
        }

        dp[n] // 返回分割后的最大和
    }
}
