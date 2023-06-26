// 找出中枢整数
// https://leetcode.cn/problems/find-the-pivot-integer
// INLINE  ../../images/math/find_the_pivot_integer.jpeg

pub struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let t = (n * n + n) / 2;
        let x = (t as f64).sqrt() as i32;
        if x * x == t {
            x
        } else {
            -1
        }
    }
}
