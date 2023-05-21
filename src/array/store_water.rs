// 蓄水
// https://leetcode.cn/problems/o8SXZn
// 这个题目原文有动画，请直接看原文

pub struct Solution;

impl Solution {
    pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        let max_k = *vat.iter().max().unwrap_or(&0);
        if max_k == 0 {
            return 0;
        }

        let mut ans = i32::MAX;
        for k in 1..=max_k {
            let mut t = 0;
            for i in 0..bucket.len() {
                t += std::cmp::max(0, (vat[i] + k - 1) / k - bucket[i]);
            }
            ans = std::cmp::min(ans, t + k);
        }
        ans
    }
}
