// 统计共同度过的日子数
// https://leetcode.cn/problems/count-days-spent-together
// INLINE  ../../images/math/count_days_spent_together.jpeg

pub struct Solution;

impl Solution {
    // 解析日期字符串为天数
    fn parse_date(date: &str) -> i32 {
        let mut iter = date.split('-');
        let month = iter.next().unwrap().parse::<i32>().unwrap();
        let day = iter.next().unwrap().parse::<i32>().unwrap();
        let mut days = 0;
        // 计算该日期距离该年1月1日有多少天
        for i in 1..month {
            days += match i {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => 28,
                #[cfg_attr(tarpaulin, skip)]
                _ => unreachable!(),
            };
        }
        days + day
    }

    // 计算共同度过的日子数
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let arrive_alice = Self::parse_date(&arrive_alice); // 将到达日期解析为天数
        let leave_alice = Self::parse_date(&leave_alice); // 将离开日期解析为天数
        let arrive_bob = Self::parse_date(&arrive_bob); // 将到达日期解析为天数
        let leave_bob = Self::parse_date(&leave_bob); // 将离开日期解析为天数
        let mut days = 0;
        // 遍历 Alice 到达和离开之间的每一天
        for i in arrive_alice..=leave_alice {
            // 如果 Bob 也在这一天到达或离开，则这一天是共同度过的
            if i >= arrive_bob && i <= leave_bob {
                days += 1;
            }
        }
        days
    }
}
