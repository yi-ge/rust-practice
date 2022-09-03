// 重新格式化字符串
// https://leetcode.cn/problems/reformat-the-string
// INLINE  ../../images/string/reformat_the_string.jpeg

pub struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let (nums, chars): (Vec<char>, Vec<char>) = s.chars().partition(|&ch| ch.is_ascii_digit());
        if (nums.len() as i32 - chars.len() as i32).abs() > 1 {
            return String::new();
        }
        let (mut long, mut short) = (chars.iter(), nums.iter());
        if chars.len() < nums.len() {
            long = nums.iter();
            short = chars.iter();
        }
        let (mut res, len) = (String::new(), nums.len() + chars.len());
        while res.len() != len {
            if let Some(&a) = long.next() {
                res.push(a);
            }
            if let Some(&b) = short.next() {
                res.push(b);
            }
        }
        res
    }
}
