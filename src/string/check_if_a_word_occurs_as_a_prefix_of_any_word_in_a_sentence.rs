// 检查单词是否为句中其他单词的前缀
// https://leetcode.cn/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence
// INLINE  ../../images/string/check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.jpeg

pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        // 将句子按空格分割成单词，并对每个单词进行遍历
        let x = sentence
            .split_whitespace()
            .enumerate()
            .fold(i32::MAX, |ret, (i, word)| {
                // 如果单词以搜索词为前缀，则返回单词在句子中的位置
                if word.starts_with(search_word.as_str()) {
                    ret.min(i as i32 + 1)
                } else {
                    ret
                }
            });
        // 如果没有找到包含搜索词前缀的单词，则返回-1；否则返回该单词在句子中的位置
        if x == i32::MAX {
            -1
        } else {
            x
        }
    }
}
