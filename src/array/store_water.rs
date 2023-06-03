// 蓄水
// https://leetcode.cn/problems/o8SXZn
// 这个题目原文有动画，请直接看原文

pub struct Solution;

impl Solution {
    pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        // 找到 vat 中的最大值
        let max_k = *vat.iter().max().unwrap_or(&0);
        // 如果最大值为 0，说明水缸已经满了，无需蓄水，直接返回 0
        if max_k == 0 {
            return 0;
        }

        // 初始化答案为最大值
        let mut ans = i32::MAX;
        // 枚举 k，k 表示加水的次数，范围为 [1, max_k]
        for k in 1..=max_k {
            // t 表示加 k 次水所需的最小次数
            let mut t = 0;
            // 遍历每个水缸
            for i in 0..bucket.len() {
                t += std::cmp::max(0, (vat[i] + k - 1) / k - bucket[i]);
            }
            // 更新答案
            ans = std::cmp::min(ans, t + k);
        }
        ans
    }
}
