// 有效的括号
// https://leetcode.cn/problems/valid-parentheses/
// INLINE  ../../images/heap/valid_parentheses.jpeg
// 解题思路：利用“栈”这个数据结构。

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect(); // 将字符串转换为字符数组
        if chars.len() == 0 {
            return true; // 空字符串是有效的
        }

        let mut stack: Vec<char> = Vec::new(); // 定义一个空栈
        for i in 0..chars.len() {
            if chars[i] == '(' {
                stack.push(')'); // 左括号入栈对应的右括号
            } else if chars[i] == '[' {
                stack.push(']'); // 左括号入栈对应的右括号
            } else if chars[i] == '{' {
                stack.push('}'); // 左括号入栈对应的右括号
            } else if stack.is_empty() || chars[i] != stack.pop().unwrap() {
                // 栈为空 或 不同于栈顶元素
                return false; // 非法情况，直接返回 false
            }
        }

        return stack.is_empty(); // 最后栈为空则合法，否则不合法
    }
}
