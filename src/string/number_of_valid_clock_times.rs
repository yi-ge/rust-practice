// 有效时间的数目
// https://leetcode.cn/problems/number-of-valid-clock-times
// INLINE  ../../images/string/number_of_valid_clock_times.jpeg

pub struct Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let f = |s: &str, m: i32| -> i32 {
            let mut cnt = 0;
            for i in 0..m {
                let a = s.chars().nth(0).unwrap() == '?'
                    || s.chars().nth(0).unwrap() as i32 - '0' as i32 == i / 10;
                let b = s.chars().nth(1).unwrap() == '?'
                    || s.chars().nth(1).unwrap() as i32 - '0' as i32 == i % 10;
                if a && b {
                    cnt += 1;
                }
            }
            cnt
        };

        f(&time[0..2], 24) * f(&time[3..5], 60)
    }
}
