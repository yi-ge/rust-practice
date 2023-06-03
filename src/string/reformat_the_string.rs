// 重新格式化字符串
// https://leetcode.cn/problems/reformat-the-string
// INLINE  ../../images/string/reformat_the_string.jpeg

pub struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        // 将字符串中的数字和字母分别存入两个 vector 中
        let (nums, chars): (Vec<char>, Vec<char>) = s.chars().partition(|&ch| ch.is_ascii_digit());
        // 判断两个 vector 的长度差是否超过 1，如果超过则无法重新格式化，返回空字符串
        if (nums.len() as i32 - chars.len() as i32).abs() > 1 {
            return String::new();
        }
        // 将长度较长的 vector 赋值给 long，长度较短的 vector 赋值给 short
        let (mut long, mut short) = (chars.iter(), nums.iter());
        if chars.len() < nums.len() {
            long = nums.iter();
            short = chars.iter();
        }
        // 定义结果字符串 res 和字符串长度 len，开始拼接字符串
        let (mut res, len) = (String::new(), nums.len() + chars.len());
        while res.len() != len {
            // 先将长字符串中的字符拼接到结果字符串 res 中
            if let Some(&a) = long.next() {
                res.push(a);
            }
            // 再将短字符串中的字符拼接到结果字符串 res 中
            if let Some(&b) = short.next() {
                res.push(b);
            }
        }
        res
    }
}
