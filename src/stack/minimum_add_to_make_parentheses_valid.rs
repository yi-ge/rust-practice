// 使括号有效的最少添加
// https://leetcode.cn/problems/minimum-add-to-make-parentheses-valid
// INLINE  ../../images/stack/minimum_add_to_make_parentheses_valid.jpeg

pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut ans = 0;
        let mut count = 0;
        for ch in s.chars() {
            if ch == '(' {
                count += 1;
            } else if count > 0 {
                count -= 1;
            } else {
                ans += 1;
            }
        }
        ans + count
    }
}
