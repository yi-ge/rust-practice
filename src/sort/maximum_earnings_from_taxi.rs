// 出租车的最大盈利
// https://leetcode.cn/problems/maximum-earnings-from-taxi
// INLINE  ../../images/sort/maximum_earnings_from_taxi.jpeg

pub struct Solution;

impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0; (n + 1) as usize];
        let mut rides = rides;
        rides.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut j = 0;
        for i in 1..=n {
            dp[i as usize] = dp[(i - 1) as usize];
            while j < rides.len() && rides[j][1] == i {
                dp[i as usize] = dp[i as usize]
                    .max(dp[rides[j][0] as usize] + rides[j][1] - rides[j][0] + rides[j][2]);
                j += 1;
            }
        }
        dp[n as usize] as i64
    }
}
