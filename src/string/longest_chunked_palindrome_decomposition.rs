// 段式回文
// https://leetcode.cn/problems/longest-chunked-palindrome-decomposition
// INLINE  ../../images/string/longest_chunked_palindrome_decomposition.jpeg

pub struct Solution;

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let n = text.len(); // 获取字符串长度
        for i in 0..n / 2 {
            // 遍历字符串的前一半
            if text[..i + 1] == text[n - i - 1..] {
                // 判断前后两部分是否相等
                // 如果相等，递归调用函数，返回值加2
                return 2 + Solution::longest_decomposition(text[i + 1..n - i - 1].to_owned());
            }
        }
        if n == 0 {
            // 如果字符串长度为0，返回0
            0
        } else {
            // 否则返回1
            1
        }
    }
}
