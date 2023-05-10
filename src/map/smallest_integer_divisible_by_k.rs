// 可被 K 整除的最小整数
// https://leetcode.cn/problems/smallest-integer-divisible-by-k
// INLINE  ../../images/map/smallest_integer_divisible_by_k.jpeg

pub struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut remainder = 0;
        for length in 1..=k {
            remainder = (remainder * 10 + 1) % k;
            if remainder == 0 {
                return length;
            }
        }
        -1
    }
}
