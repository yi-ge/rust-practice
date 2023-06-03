// 子串能表示从 1 到 N 数字的二进制串
// https://leetcode.cn/problems/binary-string-with-substrings-representing-1-to-n
// INLINE  ../../images/string/binary_string_with_substrings_representing_1_to_n.jpeg

pub struct Solution;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let mut n = n; // 初始化n
        while n > 0 {
            // 循环判断1到N能否由s中的子串表示
            if !s.contains(&format!("{:b}", n)) {
                // 使用format!将n转换为二进制字符串，并在s中查找该字符串
                return false; // 如果找不到该字符串，则说明不能由s中的子串表示，返回false
            }
            n -= 1; // 继续判断1到N-1能否由s中的子串表示
        }
        true // 如果1到N都能由s中的子串表示，则返回true
    }
}
