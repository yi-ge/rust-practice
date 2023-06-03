// 最长等差数列
// https://leetcode.cn/problems/longest-arithmetic-subsequence
// INLINE  ../../images/array/longest_arithmetic_subsequence.jpeg

use std::{cmp::max, iter::repeat};

pub struct Solution;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        // 找到最小值和最大值
        let min_item = *nums.iter().min().unwrap();
        let max_item = *nums.iter().max().unwrap();
        // 计算出数字范围
        let diff = max_item - min_item;
        let mut ans = 1;
        // 枚举差值
        for d in -diff..=diff {
            // 初始化f数组，表示以nums中的某个数字为结尾，公差为d的等差数列的最大长度
            let mut f: Vec<i32> = repeat(-1).take((max_item + 1) as usize).collect();
            for &num in &nums {
                // 计算出前一个数字
                let prev = num - d;
                // 如果前一个数字存在且可以作为等差数列的前一个数字
                if prev >= min_item && prev <= max_item && f[prev as usize] != -1 {
                    // 更新f数组
                    f[num as usize] = max(f[num as usize], f[prev as usize] + 1);
                    // 更新最大长度
                    ans = max(ans, f[num as usize]);
                }
                // 如果前一个数字不存在或者不能作为等差数列的前一个数字，那么当前数字的等差数列长度至少为1
                f[num as usize] = max(f[num as usize], 1);
            }
        }
        ans
    }
}
