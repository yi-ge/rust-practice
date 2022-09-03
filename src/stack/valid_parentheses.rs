// 有效的括号
// https://leetcode.cn/problems/valid-parentheses/
// INLINE  ../../images/heap/valid_parentheses.jpeg
// 解题思路：利用“栈”这个数据结构。

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 0 {
            return true;
        }

        let mut stack: Vec<char> = Vec::new();
        for i in 0..chars.len() {
            if chars[i] == '(' {
                stack.push(')');
            } else if chars[i] == '[' {
                stack.push(']');
            } else if chars[i] == '{' {
                stack.push('}');
            } else if stack.is_empty() || chars[i] != stack.pop().unwrap() {
                // 栈为空 或 不同于栈顶元素
                return false;
            }
        }

        return stack.is_empty();
    }
}
