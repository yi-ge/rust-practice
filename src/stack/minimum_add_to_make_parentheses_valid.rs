// 使括号有效的最少添加
// https://leetcode.cn/problems/minimum-add-to-make-parentheses-valid
// INLINE  ../../images/stack/minimum_add_to_make_parentheses_valid.jpeg

pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut ans = 0; // 记录需要添加的括号数
        let mut count = 0; // 记录当前左括号的数量
        for ch in s.chars() {
            // 遍历字符串中的字符
            if ch == '(' {
                // 如果是左括号，计数器加1
                count += 1;
            } else if count > 0 {
                // 如果是右括号且当前有左括号可以匹配，计数器减1
                count -= 1;
            } else {
                // 如果是右括号且当前没有左括号可以匹配，需要添加一个左括号来匹配
                ans += 1;
            }
        }
        ans + count // 最后还需要将剩余的左括号加到答案中
    }
}
