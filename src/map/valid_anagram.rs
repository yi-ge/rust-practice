// 有效的字母异位词
// https://leetcode.cn/problems/valid-anagram
// INLINE  ../../images/map/valid_anagram.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // 如果两个字符串长度不同，直接返回false
        if s.len() != t.len() {
            return false;
        }

        // 使用哈希表记录s中每个字符出现的次数
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        // 遍历t中的每个字符，更新map中对应字符的次数
        for c in t.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;

            // 如果某个字符的次数小于0，说明t中出现了s中没有的字符，直接返回false
            if *count < 0 {
                return false;
            }
        }

        // 如果遍历完t后没有返回false，说明t是s的字母异位词，返回true
        true
    }
}
