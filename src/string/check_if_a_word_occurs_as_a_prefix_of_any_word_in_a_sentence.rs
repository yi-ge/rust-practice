// 检查单词是否为句中其他单词的前缀
// https://leetcode.cn/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence
// INLINE  ../../images/string/check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.jpeg

pub struct Solution {}

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
      let x = sentence.split_whitespace().enumerate().fold(i32::MAX, |ret, (i, word)| if word.starts_with(search_word.as_str()) { ret.min(i as i32 + 1) } else { ret });
      if x == i32::MAX { -1 } else { x }
    }
}