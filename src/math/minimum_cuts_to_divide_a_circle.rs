// 分割圆的最少切割次数
// https://leetcode.cn/problems/minimum-cuts-to-divide-a-circle
// INLINE  ../../images/math/minimum_cuts_to_divide_a_circle.jpeg

pub struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if n % 2 == 0 {
            return n / 2;
        }
        return n;
    }
}
