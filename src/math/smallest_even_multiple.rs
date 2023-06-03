// 最小偶倍数
// https://leetcode.cn/problems/smallest-even-multiple
// INLINE  ../../images/math/smallest_even_multiple.jpeg

pub struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        // 如果 n 是偶数，直接返回 n
        if n % 2 == 0 {
            n
        } else {
            // 如果 n 是奇数，返回 2 * n，这是因为偶数一定可以被 2 整除，而 2 * n 可以保证是偶数且是 n 的倍数
            2 * n
        }
    }
}
