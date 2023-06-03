// 可被 K 整除的最小整数
// https://leetcode.cn/problems/smallest-integer-divisible-by-k
// INLINE  ../../images/map/smallest_integer_divisible_by_k.jpeg

pub struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut remainder = 0; // 余数初始化为0
        for length in 1..=k {
            remainder = (remainder * 10 + 1) % k; // 计算新的余数
            if remainder == 0 {
                // 如果余数为0，说明可以整除，返回当前长度
                return length;
            }
        }
        -1 // 如果循环结束仍未找到可以整除的数，返回-1
    }
}
