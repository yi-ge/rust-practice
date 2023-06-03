// 字母异位词分组
// https://leetcode.cn/problems/group-anagrams
// INLINE  ../../images/array/group_anagrams.jpeg
// 解题思路：利用map记录已经排序的单个字符串对应的字符串，每次判断字符串排序后的字符是否在map中，如果在，则insert到map。

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 创建一个空的HashMap，用于存储排序后的字符串及其对应的原始字符串
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        // 创建一个空的二维数组，用于存储分组后的结果
        let mut result: Vec<Vec<String>> = Vec::new();

        // 遍历给定的字符串集合
        for i in 0..strs.len() {
            // 将当前字符串转换为字符数组，并进行排序
            let mut chars = vec![];
            for c in strs[i].chars() {
                chars.push(c);
            }
            chars.sort();

            // 将排序后的字符数组转换为字符串
            let str = chars.into_iter().collect();

            // 判断当前排序后的字符串是否已经存在于map中，如果存在，则将原始字符串添加到对应的值中，否则将当前键值对添加到map中
            if let Some(v) = map.get(&str) {
                let mut value = v.to_vec();
                value.push(strs[i].clone());
                map.insert(str, value);
            } else {
                map.insert(str, vec![strs[i].clone()]);
            }
        }

        // 遍历map中的所有值，并将其添加到result中
        for val in map.values() {
            result.push(val.to_vec());
        }

        // 返回分组后的结果
        result
    }
}
