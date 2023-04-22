// 最小偶倍数
// https://leetcode.cn/problems/smallest-even-multiple
// INLINE  ../../images/math/smallest_even_multiple.jpeg

pub struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            n
        } else {
            2 * n
        }
    }
}
