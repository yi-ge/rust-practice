// 二进制字符串前缀一致的次数
// https://leetcode.cn/problems/number-of-times-binary-string-is-prefix-aligned
// INLINE  ../../images/array/number_of_times_binary_string_is_prefix_aligned.jpeg

pub struct Solution;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut right = 0;
        let mut count = 0;
        for i in 0..flips.len() {
            right = right.max(flips[i]);
            if right == i as i32 + 1 {
                count += 1;
            }
        }
        count
    }
}
