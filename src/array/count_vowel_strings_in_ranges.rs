// 统计范围内的元音字符串数
// https://leetcode.cn/problems/count-vowel-strings-in-ranges
// INLINE  ../../images/array/count_vowel_strings_in_ranges.jpeg

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sums: Vec<i32> = vec![0; words.len() + 1];
        for i in 0..words.len() {
            let value = if Solution::is_vowel_string(&words[i]) {
                1
            } else {
                0
            };
            prefix_sums[i + 1] = prefix_sums[i] + value;
        }

        let mut ans: Vec<i32> = vec![0; queries.len()];
        for i in 0..queries.len() {
            let start = queries[i][0] as usize;
            let end = queries[i][1] as usize;
            ans[i] = prefix_sums[end + 1] - prefix_sums[start];
        }

        ans
    }

    // 检查字符串是否为元音字符串。
    fn is_vowel_string(word: &String) -> bool {
        Solution::is_vowel_letter(word.chars().nth(0).unwrap())
            && Solution::is_vowel_letter(word.chars().last().unwrap())
    }

    // 检查字母是否为元音字母。
    fn is_vowel_letter(c: char) -> bool {
        let vowel_letters: HashSet<char> = HashSet::from_iter(vec!['a', 'e', 'i', 'o', 'u']);
        return vowel_letters.contains(&c);
    }
}
