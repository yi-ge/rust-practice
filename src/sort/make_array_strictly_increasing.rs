// 使数组严格递增
// https://leetcode.cn/problems/make-array-strictly-increasing
// INLINE  ../../images/sort/make_array_strictly_increasing.jpeg

use std::cmp;

const INF: i32 = 0x3f3f3f3f;

pub struct Solution;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2_sorted = arr2.clone();
        arr2_sorted.sort(); // 将arr2排序
        arr2_sorted.dedup(); // 去重
        let n = arr1.len(); // arr1的长度
        let m = arr2_sorted.len(); // 去重后的arr2的长度
        let mut dp = vec![vec![INF; cmp::min(m, n) + 1]; n + 1]; // dp数组，用于动态规划
        dp[0][0] = -1; // 初始化
        for i in 1..=n {
            for j in 0..=cmp::min(i, m) {
                // 如果当前元素比序列中最后一个元素大
                if arr1[i - 1] > dp[i - 1][j] {
                    dp[i][j] = arr1[i - 1]; // 直接将当前元素加入序列
                }
                if j > 0 && dp[i - 1][j - 1] != INF {
                    // 找到大于dp[i - 1][j - 1]的最小元素
                    let idx = arr2_sorted
                        .binary_search_by(|probe| probe.cmp(&(dp[i - 1][j - 1] + 1)))
                        .unwrap_or_else(|err| err);
                    if idx < arr2_sorted.len() {
                        dp[i][j] = cmp::min(dp[i][j], arr2_sorted[idx]); // 更新dp值
                    }
                }
                if i == n && dp[i][j] != INF {
                    // 如果已经遍历完arr1并且存在符合要求的序列
                    return j as i32; // 返回最小的步数
                }
            }
        }
        -1 // 没有符合要求的序列，返回-1
    }
}
