use std::collections::HashMap;
// 统计出现过一次的公共字符串
// https://leetcode.cn/problems/count-common-words-with-one-occurrence
// INLINE  ../../images/array/count_common_words_with_one_occurrence.jpeg

pub struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map1: HashMap<String, i32> = HashMap::new();
        let mut map2: HashMap<String, i32> = HashMap::new();

        for w in words1 {
            *map1.entry(w).or_insert(0) += 1;
        }

        for w in words2 {
            *map2.entry(w).or_insert(0) += 1;
        }

        let mut res = 0;
        for (w, cnt1) in map1 {
            if cnt1 == 1 && map2.get(&w) == Some(&1) {
                res += 1;
            }
        }
        res
    }
}
