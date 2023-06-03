// 检查替换后的词是否有效
// https://leetcode.cn/problems/check-if-word-is-valid-after-substitutions
// INLINE  ../../images/stack/check_if_word_is_valid_after_substitutions.jpeg
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new(); // 初始化一个栈
        for c in s.chars() {
            // 遍历字符串中的每个字符
            if c == 'c' {
                // 如果当前字符是‘c’
                if stack.len() < 2 {
                    // 如果栈的长度小于2，说明无法匹配
                    return false; // 返回false
                }
                if stack.pop().unwrap() != 'b' || stack.pop().unwrap() != 'a' {
                    // 如果栈中的顶部两个元素不是‘a’和‘b’，说明无法匹配
                    return false; // 返回false
                }
            } else {
                // 如果当前字符不是‘c’
                stack.push(c); // 将当前字符入栈
            }
        }
        stack.is_empty() // 最后判断栈是否为空，如果为空则说明所有的字符都匹配成功，返回true，否则返回false
    }
}
