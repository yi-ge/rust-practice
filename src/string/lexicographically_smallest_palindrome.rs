// 字典序最小回文串
// https://leetcode.cn/problems/lexicographically-smallest-palindrome
// INLINE  ../../images/string/lexicographically_smallest_palindrome.jpeg

pub struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            if s[left] != s[right] {
                s[left] = std::cmp::min(s[left], s[right]);
                s[right] = std::cmp::min(s[left], s[right]);
            }
            left += 1;
            right -= 1;
        }

        s.into_iter().collect()
    }
}
