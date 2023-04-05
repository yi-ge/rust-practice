// 公因子的数目
// https://leetcode.cn/problems/number-of-common-factors
// INLINE  ../../images/math/number_of_common_factors.jpeg

pub struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut count = 0;
        let mut i = 1;
        while i * i <= a {
            if a % i == 0 {
                if b % i == 0 {
                    count += 1;
                }
                if b % (a / i) == 0 && a / i != i {
                    count += 1;
                }
            }
            i += 1;
        }
        count
    }
}
