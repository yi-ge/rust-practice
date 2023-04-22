// 驼峰式匹配
// https://leetcode.cn/problems/camelcase-matching
// INLINE  ../../images/tree/camelcase_matching.jpeg

pub struct Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result = Vec::new();
        for query in queries {
            let query_chars = query.chars();
            let mut pattern_chars = pattern.chars();
            let mut pattern_char = pattern_chars.next();
            let mut match_flag = true;
            for query_char in query_chars {
                if query_char.is_uppercase() {
                    if Some(query_char) != pattern_char {
                        match_flag = false;
                        break;
                    } else {
                        pattern_char = pattern_chars.next();
                    }
                } else {
                    if Some(query_char) == pattern_char {
                        pattern_char = pattern_chars.next();
                    }
                }
            }
            if pattern_char.is_some() {
                match_flag = false;
            }
            result.push(match_flag);
        }
        result
    }
}
