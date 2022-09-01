// 有效的字母异位词
// https://leetcode.cn/problems/valid-anagram
// INLINE  ../../images/map/valid_anagram.jpeg

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;

            if *count < 0 {
                return false;
            }
        }

        true
    }
}
