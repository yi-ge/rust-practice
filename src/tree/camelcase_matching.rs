// 驼峰式匹配
// https://leetcode.cn/problems/camelcase-matching
// INLINE  ../../images/tree/camelcase_matching.jpeg

pub struct Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result = Vec::new(); // 存储结果的向量
        for query in queries {
            // 遍历所有查询单词
            let query_chars = query.chars(); // 将查询单词转换为字符迭代器
            let mut pattern_chars = pattern.chars(); // 将匹配模式转换为字符迭代器
            let mut pattern_char = pattern_chars.next(); // 获取匹配模式的第一个字符
            let mut match_flag = true; // 标志位，表示当前查询单词是否匹配模式
            for query_char in query_chars {
                // 遍历查询单词的所有字符
                if query_char.is_uppercase() {
                    // 如果当前字符是大写字母
                    if Some(query_char) != pattern_char {
                        // 如果当前字符和匹配模式的当前字符不匹配
                        match_flag = false; // 将标志位设为false
                        break; // 退出循环
                    } else {
                        // 如果当前字符和匹配模式的当前字符匹配
                        pattern_char = pattern_chars.next(); // 将匹配模式的当前字符设为下一个字符
                    }
                } else {
                    // 如果当前字符不是大写字母
                    if Some(query_char) == pattern_char {
                        // 如果当前字符和匹配模式的当前字符匹配
                        pattern_char = pattern_chars.next(); // 将匹配模式的当前字符设为下一个字符
                    }
                }
            }
            if pattern_char.is_some() {
                // 如果匹配模式还有剩余字符
                match_flag = false; // 将标志位设为false
            }
            result.push(match_flag); // 将标志位加入结果向量中
        }
        result // 返回结果向量
    }
}
