// 检查替换后的词是否有效
// https://leetcode.cn/problems/check-if-word-is-valid-after-substitutions
// INLINE  ../../images/stack/check_if_word_is_valid_after_substitutions.jpeg

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c == 'c' {
                if stack.len() < 2 {
                    return false;
                }
                if stack.pop().unwrap() != 'b' || stack.pop().unwrap() != 'a' {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}
