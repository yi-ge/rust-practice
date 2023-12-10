// 爬楼梯
// https://leetcode.cn/problems/climbing-stairs
// INLINE  ../../images/search/climbing_stairs.jpeg

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}
