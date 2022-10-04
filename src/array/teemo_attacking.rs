// 提莫攻击
// https://leetcode.cn/problems/teemo-attacking/
// INLINE  ../../images/array/teemo_attacking.jpeg

pub struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut res = 0;
        for i in 1..time_series.len() {
            if time_series[i - 1] + duration <= time_series[i] {
                res += duration;
            } else {
                res += time_series[i] - time_series[i - 1];
            }
        }

        res + duration
    }
}
