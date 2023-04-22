// 统计字典序元音字符串的数目
// https://leetcode.cn/problems/count-sorted-vowel-strings
// INLINE  ../../images/math/count_sorted_vowel_strings.jpeg

pub struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        (n + 4) * (n + 3) * (n + 2) * (n + 1) / 24
    }
}
