// 最长数对链
// https://leetcode.cn/problems/maximum-length-of-pair-chain
// INLINE  ../../images/array/maximum_length_of_pair_chain.jpeg
// 解题思路：贪心

pub struct Solution {}

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        let mut curr = i32::MIN;
        let mut ans = 0;

        pairs.sort_by(|a, b| a[1].cmp(&b[1]));

        for pair in pairs {
            if pair[0] > curr {
                curr = pair[1];
                ans += 1;
            }
        }

        ans
    }
}
