// 子串能表示从 1 到 N 数字的二进制串
// https://leetcode.cn/problems/binary-string-with-substrings-representing-1-to-n
// INLINE  ../../images/string/binary_string_with_substrings_representing_1_to_n.jpeg

pub struct Solution;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if !s.contains(&format!("{:b}", n)) {
                return false;
            }
            n -= 1;
        }
        true
    }
}
