// 最长等差数列
// https://leetcode.cn/problems/longest-arithmetic-subsequence
// INLINE  ../../images/array/longest_arithmetic_subsequence.jpeg

use std::{cmp::max, iter::repeat};

pub struct Solution;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let min_item = *nums.iter().min().unwrap();
        let max_item = *nums.iter().max().unwrap();
        let diff = max_item - min_item;
        let mut ans = 1;
        for d in -diff..=diff {
            let mut f: Vec<i32> = repeat(-1).take((max_item + 1) as usize).collect();
            for &num in &nums {
                let prev = num - d;
                if prev >= min_item && prev <= max_item && f[prev as usize] != -1 {
                    f[num as usize] = max(f[num as usize], f[prev as usize] + 1);
                    ans = max(ans, f[num as usize]);
                }
                f[num as usize] = max(f[num as usize], 1);
            }
        }
        ans
    }
}
