// 出租车的最大盈利
// https://leetcode.cn/problems/maximum-earnings-from-taxi
// INLINE  ../../images/sort/maximum_earnings_from_taxi.jpeg

pub struct Solution;

impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0i64; (n + 1) as usize];
        let mut rides: Vec<Vec<i64>> = rides
            .into_iter()
            .map(|ride| ride.into_iter().map(|x| x as i64).collect())
            .collect();

        rides.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut j = 0usize;
        for i in 1..=n as usize {
            dp[i] = dp[i - 1];
            while j < rides.len() && rides[j][1] == i as i64 {
                dp[i] =
                    dp[i].max(dp[rides[j][0] as usize] + rides[j][1] - rides[j][0] + rides[j][2]);
                j += 1;
            }
        }
        dp[n as usize]
    }
}
