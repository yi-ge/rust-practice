// 构造限制重复的字符串
// https://leetcode.cn/problems/construct-string-with-repeat-limit
// INLINE  ../../images/string/construct_string_with_repeat_limit.jpeg

pub struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut count: [i32; 26] = [0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        let mut ret = String::new();
        let mut m = 0;
        let mut i = 25;
        let mut j = 24;
        while i >= 0 && j >= 0 {
            if count[i as usize] == 0 {
                m = 0;
                i -= 1;
            } else if m < repeat_limit {
                count[i as usize] -= 1;
                ret.push((b'a' + i as u8) as char);
                m += 1;
            } else if j >= i || count[j as usize] == 0 {
                j -= 1;
            } else {
                count[j as usize] -= 1;
                ret.push((b'a' + j as u8) as char);
                m = 0;
            }
        }
        ret
    }
}
