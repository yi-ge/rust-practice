// 按分隔符拆分字符串
// https://leetcode.cn/problems/split-strings-by-separator
// INLINE  ../../images/array/split_strings_by_separator.jpeg

pub struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for word in words {
            let ss = word.split(separator);
            for sub in ss {
                if !sub.is_empty() {
                    res.push(sub.to_string());
                }
            }
        }
        res
    }
}
