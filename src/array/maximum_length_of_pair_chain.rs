// 最长数对链
// https://leetcode.cn/problems/maximum-length-of-pair-chain
// INLINE  ../../images/array/maximum_length_of_pair_chain.jpeg
// 解题思路：贪心

pub struct Solution;

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        let mut curr = i32::MIN; // 当前数对的右端点
        let mut ans = 0; // 最长数对链的长度

        pairs.sort_by(|a, b| a[1].cmp(&b[1])); // 按数对的右端点升序排序

        for pair in pairs {
            if pair[0] > curr {
                // 如果当前数对左端点大于上一个数对的右端点
                curr = pair[1]; // 更新当前数对的右端点
                ans += 1; // 长度加一
            }
        }

        ans // 返回最长数对链的长度
    }
}
