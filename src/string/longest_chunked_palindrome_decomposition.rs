// 段式回文
// https://leetcode.cn/problems/longest-chunked-palindrome-decomposition
// INLINE  ../../images/string/longest_chunked_palindrome_decomposition.jpeg

pub struct Solution;

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let n = text.len();
        for i in 0..n / 2 {
            if text[..i + 1] == text[n - i - 1..] {
                return 2 + Solution::longest_decomposition(text[i + 1..n - i - 1].to_owned());
            }
        }
        if n == 0 {
            0
        } else {
            1
        }
    }
}
