// 字母异位词分组
// https://leetcode.cn/problems/group-anagrams
// INLINE  ../../images/array/group_anagrams.jpeg
// 解题思路：利用map记录已经排序的单个字符串对应的字符串，每次判断字符串排序后的字符是否在map中，如果在，则insert到map。

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut result: Vec<Vec<String>> = Vec::new();

        for i in 0..strs.len() {
            let mut chars = vec![];
            for c in strs[i].chars() {
                chars.push(c);
            }
            chars.sort();

            let str = chars.into_iter().collect();

            if let Some(v) = map.get(&str) {
                let mut value = v.to_vec();
                value.push(strs[i].clone());
                map.insert(str, value);
            } else {
                map.insert(str, vec![strs[i].clone()]);
            }
        }

        for val in map.values() {
            result.push(val.to_vec());
        }

        result
    }
}
