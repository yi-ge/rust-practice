// 检查二进制字符串字段
// https://leetcode.cn/problems/check-if-binary-string-has-at-most-one-segment-of-ones
// INLINE  ../../images/string/check_if_binary_string_has_at_most_one_segment_of_ones.jpeg

pub struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !s.contains("01")
    }
}
