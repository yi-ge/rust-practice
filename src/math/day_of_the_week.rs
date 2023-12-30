// 一周中的第几天
// https://leetcode.cn/problems/day-of-the-week
// INLINE  ../../images/math/day_of_the_week.jpeg

pub struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let week = vec![
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        let month_days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];
        let mut days = 365 * (year - 1971) + (year - 1969) / 4;
        for i in 0..(month - 1) {
            days += month_days[i as usize];
        }
        if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0) && month > 2 {
            days += 1;
        }
        days += day;
        return week[(days + 3) as usize % 7].to_string();
    }
}
