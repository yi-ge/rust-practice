// 检查二进制字符串字段
// https://leetcode.cn/problems/check-if-binary-string-has-at-most-one-segment-of-ones
// INLINE  ../../images/string/check_if_binary_string_has_at_most_one_segment_of_ones.jpeg

pub struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        // 如果字符串中包含"01"，说明有至少两段连续的1出现，返回false
        !s.contains("01")
    }
}
