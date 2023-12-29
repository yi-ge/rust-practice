// 购买两块巧克力
// https://leetcode.cn/problems/buy-two-chocolates
// INLINE  ../../images/sort/buy_two_chocolates.jpeg

pub struct Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut fi = i32::MAX;
        let mut se = i32::MAX;
        for p in prices {
            if p < fi {
                se = fi;
                fi = p;
            } else if p < se {
                se = p;
            }
        }
        if money < fi + se {
            money
        } else {
            money - fi - se
        }
    }
}
