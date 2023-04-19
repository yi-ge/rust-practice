// 分隔数组以得到最大和
// https://leetcode.cn/problems/partition-array-for-maximum-sum
// INLINE  ../../images/array/partition_array_for_maximum_sum.jpeg

pub struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![0; n + 1]; // Dynamic programming table to store the maximum sum

        // Iterate through the array
        for i in 1..=n {
            let mut max_val_in_partition = arr[i - 1]; // Initialize max value in the current partition

            // Iterate in reverse from current position (i) up to k elements back
            let mut j = i.saturating_sub(1);
            while j + k as usize >= i {
                // Update the maximum sum by comparing the current value to the sum of the previous partition
                dp[i] = std::cmp::max(dp[i], dp[j] + max_val_in_partition * (i - j) as i32);

                // Update max_val_in_partition if there is still an element in the partition
                if j > 0 {
                    max_val_in_partition = std::cmp::max(max_val_in_partition, arr[j - 1]);
                    j -= 1;
                } else {
                    break;
                }
            }
        }

        dp[n] // Return the maximum sum after partitioning
    }
}
