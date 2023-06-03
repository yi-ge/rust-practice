// 提莫攻击
// https://leetcode.cn/problems/teemo-attacking/
// INLINE  ../../images/array/teemo_attacking.jpeg

pub struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut res = 0; // 初始化中毒总时间为0
        for i in 1..time_series.len() {
            // 遍历时间序列中的每个时间点
            if time_series[i - 1] + duration <= time_series[i] {
                // 如果前一个时间点的中毒效果已经结束了
                res += duration; // 则将中毒总时间加上本次攻击的持续时间
            } else {
                // 否则，前一个时间点的中毒效果还在持续，需要计算重叠的时间
                res += time_series[i] - time_series[i - 1]; // 将中毒总时间加上两个时间点的时间差
            }
        }

        res + duration // 将最后一次攻击的持续时间加上中毒总时间即为总中毒时间
    }
}
