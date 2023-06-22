// 可被三整除的最大和
// https://leetcode.cn/problems/greatest-sum-divisible-by-three
// INLINE  ../../images/sort/greatest_sum_divisible_by_three.jpeg

pub struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, 0, 0];
        for num in nums {
            let mut dp_tmp = dp.clone();
            for i in 0..3 {
                let index = (num + dp[i]) % 3;
                dp_tmp[index as usize] = dp_tmp[index as usize].max(num + dp[i]);
            }
            dp = dp_tmp;
        }
        dp[0]
    }
}
