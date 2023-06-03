// 有效时间的数目
// https://leetcode.cn/problems/number-of-valid-clock-times
// INLINE  ../../images/string/number_of_valid_clock_times.jpeg

pub struct Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        // 定义一个闭包
        let f = |s: &str, m: i32| -> i32 {
            let mut cnt = 0;
            // 遍历 0 到 m-1 的数字
            for i in 0..m {
                // 判断小时或分钟的第一个字符是否为 '?'
                // 或者是否等于 i/10，即十位上的数字
                let a = s.chars().nth(0).unwrap() == '?'
                    || s.chars().nth(0).unwrap() as i32 - '0' as i32 == i / 10;
                // 判断小时或分钟的第二个字符是否为 '?'
                // 或者是否等于 i%10，即个位上的数字
                let b = s.chars().nth(1).unwrap() == '?'
                    || s.chars().nth(1).unwrap() as i32 - '0' as i32 == i % 10;
                // 如果两个字符都符合要求，计数器加 1
                if a && b {
                    cnt += 1;
                }
            }
            cnt
        };
        // 分别计算小时和分钟的有效数目，相乘即可得到总数目
        f(&time[0..2], 24) * f(&time[3..5], 60)
    }
}
