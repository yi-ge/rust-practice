// 统计整数数目
// https://leetcode.cn/problems/count-of-integers
// INLINE  ../../images/math/count_of_integers.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let mod_val = 1_000_000_007;
        let m = num2.len();
        let up_limit: Vec<i32> = num2
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let down_limit: Vec<i32> = format!("{:0>width$}", num1, width = m)
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut memo: HashMap<(usize, i32, bool, bool, bool), i32> = HashMap::new();

        fn f(
            i: usize,
            s: i32,
            valid: bool,
            dlimit: bool,
            ulimit: bool,
            m: usize,
            up_limit: &Vec<i32>,
            down_limit: &Vec<i32>,
            memo: &mut HashMap<(usize, i32, bool, bool, bool), i32>,
            mod_val: i32,
            min_sum: i32,
            max_sum: i32,
        ) -> i32 {
            if i == m {
                return if valid && s >= min_sum && s <= max_sum {
                    1
                } else {
                    0
                };
            }
            let key = (i, s, valid, dlimit, ulimit);
            if let Some(&val) = memo.get(&key) {
                return val;
            }
            let down = if dlimit { down_limit[i] } else { 0 };
            let up = if ulimit { up_limit[i] } else { 9 };
            let mut ans = 0;
            for d in down..=up {
                ans =
                    (ans + f(
                        i + 1,
                        s + d,
                        valid || d != 0,
                        dlimit && d == down,
                        ulimit && d == up,
                        m,
                        up_limit,
                        down_limit,
                        memo,
                        mod_val,
                        min_sum,
                        max_sum,
                    )) % mod_val;
            }
            memo.insert(key, ans);
            ans
        }

        f(
            0,
            0,
            false,
            true,
            true,
            m,
            &up_limit,
            &down_limit,
            &mut memo,
            mod_val,
            min_sum,
            max_sum,
        )
    }
}
