// 构造有效字符串的最少插入数
// https://leetcode.cn/problems/minimum-additions-to-make-valid-string
// INLINE  ../../images/stack/minimum_additions_to_make_valid_string.jpeg

pub struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let n = word.len();
        let mut cnt = 1;
        for i in 1..n {
            if word.chars().nth(i) <= word.chars().nth(i - 1) {
                cnt += 1;
            }
        }
        return cnt * 3 - n as i32;
    }
}
