// 删除子串后的字符串最小长度
// https://leetcode.cn/problems/minimum-string-length-after-removing-substrings
// INLINE  ../../images/stack/minimum_string_length_after_removing_substrings.jpeg

pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = vec![];
        for c in s.chars() {
            stack.push(c);
            let len = stack.len();
            if len >= 2
                && (stack[len - 2] == 'A' && stack[len - 1] == 'B'
                    || stack[len - 2] == 'C' && stack[len - 1] == 'D')
            {
                stack.pop();
                stack.pop();
            }
        }
        stack.len() as i32
    }
}
