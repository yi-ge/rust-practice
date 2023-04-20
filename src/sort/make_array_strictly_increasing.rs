// 使数组严格递增
// https://leetcode.cn/problems/make-array-strictly-increasing
// INLINE  ../../images/sort/make_array_strictly_increasing.jpeg

use std::cmp;

const INF: i32 = 0x3f3f3f3f;

pub struct Solution;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2_sorted = arr2.clone();
        arr2_sorted.sort();
        arr2_sorted.dedup();
        let n = arr1.len();
        let m = arr2_sorted.len();
        let mut dp = vec![vec![INF; cmp::min(m, n) + 1]; n + 1];
        dp[0][0] = -1;
        for i in 1..=n {
            for j in 0..=cmp::min(i, m) {
                // If the current element is greater than the last element in the sequence
                if arr1[i - 1] > dp[i - 1][j] {
                    dp[i][j] = arr1[i - 1];
                }
                if j > 0 && dp[i - 1][j - 1] != INF {
                    // Find the smallest element strictly greater than dp[i - 1][j - 1]
                    let idx = arr2_sorted
                        .binary_search_by(|probe| probe.cmp(&(dp[i - 1][j - 1] + 1)))
                        .unwrap_or_else(|err| err);
                    if idx < arr2_sorted.len() {
                        dp[i][j] = cmp::min(dp[i][j], arr2_sorted[idx]);
                    }
                }
                if i == n && dp[i][j] != INF {
                    return j as i32;
                }
            }
        }
        -1
    }
}
