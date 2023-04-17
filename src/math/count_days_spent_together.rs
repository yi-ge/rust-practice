// 统计共同度过的日子数
// https://leetcode.cn/problems/count-days-spent-together
// INLINE  ../../images/math/count_days_spent_together.jpeg

pub struct Solution;

impl Solution {
    fn parse_date(date: &str) -> i32 {
        let mut iter = date.split('-');
        let month = iter.next().unwrap().parse::<i32>().unwrap();
        let day = iter.next().unwrap().parse::<i32>().unwrap();
        let mut days = 0;
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
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let arrive_alice = Self::parse_date(&arrive_alice);
        let leave_alice = Self::parse_date(&leave_alice);
        let arrive_bob = Self::parse_date(&arrive_bob);
        let leave_bob = Self::parse_date(&leave_bob);
        let mut days = 0;
        for i in arrive_alice..=leave_alice {
            if i >= arrive_bob && i <= leave_bob {
                days += 1;
            }
        }
        days
    }
}
