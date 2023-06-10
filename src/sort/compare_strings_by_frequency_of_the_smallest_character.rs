// 比较字符串最小字母出现频次
// https://leetcode.cn/problems/compare-strings-by-frequency-of-the-smallest-character
// INLINE  ../../images/sort/compare_strings_by_frequency_of_the_smallest_character.jpeg

pub struct Solution;

impl Solution {
    fn f(s: &str) -> usize {
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[(c as u8 - 'a' as u8) as usize] += 1;
        }
        for i in 0..count.len() {
            if count[i] != 0 {
                return count[i];
            }
        }
        0
    }
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut words_count = vec![0; 11];
        for word in words {
            let count = Solution::f(&word);
            words_count[count] += 1;
        }

        let mut words_count_sum = vec![0; 11];
        for i in 1..words_count.len() {
            words_count_sum[i] = words_count_sum[i - 1] + words_count[i];
        }

        let mut result = vec![];
        for query in queries {
            let count = Solution::f(&query);
            result.push(words_count_sum[words_count_sum.len() - 1] - words_count_sum[count]);
        }
        result
    }
}