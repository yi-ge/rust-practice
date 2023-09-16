// 打家劫舍
// https://leetcode.cn/problems/house-robber
// INLINE  ../../images/array/house_robber.jpeg

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        dp[0] = 0;
        dp[1] = nums[0];
        for i in 2..=nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i - 1]);
        }
        dp[nums.len()]
    }
}
