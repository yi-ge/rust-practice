// 最大字符串配对数目
// https://leetcode.cn/problems/find-maximum-number-of-string-pairs
// INLINE  ../../images/array/find_maximum_number_of_string_pairs.jpeg

pub struct Solution;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut ans = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let word_i = words[i].as_bytes();
                let word_j = words[j].as_bytes();
                if word_i[0] == word_j[1] && word_i[1] == word_j[0] {
                    ans += 1;
                }
            }
        }
        ans
    }
}
