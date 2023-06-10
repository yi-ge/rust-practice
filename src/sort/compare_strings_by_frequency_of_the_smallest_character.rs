// 比较字符串最小字母出现频次
// https://leetcode.cn/problems/compare-strings-by-frequency-of-the-smallest-character
// INLINE  ../../images/sort/compare_strings_by_frequency_of_the_smallest_character.jpeg

pub struct Solution;

impl Solution {
    /// 计算字符串中最小字符的频率
    fn f(s: &str) -> usize {
        // 计算每个字符的频率
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[(c as u8 - 'a' as u8) as usize] += 1;
        }

        // 返回第一个非零频率
        for i in 0..count.len() {
            if count[i] != 0 {
                return count[i];
            }
        }
        0
    }

    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        // 计算每个单词的最小字符的频率
        let mut words_count = vec![0; 11];
        for word in words {
            let count = Solution::f(&word);
            words_count[count] += 1;
        }

        // 计算累积频率
        let mut words_count_sum = vec![0; 11];
        for i in 1..words_count.len() {
            words_count_sum[i] = words_count_sum[i - 1] + words_count[i];
        }

        // 计算每个查询的最小字符的频率
        let mut result = vec![];
        for query in queries {
            let count = Solution::f(&query);
            // 计算频率更小的单词的数量
            result.push(words_count_sum[words_count_sum.len() - 1] - words_count_sum[count]);
        }
        result
    }
}