// 检查相同字母间的距离
// https://leetcode.cn/problems/check-distances-between-same-letters
// INLINE  ../../images/array/check_distances_between_same_letters.jpeg

pub struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        // 创建一个长度为26的Option<i32>类型的向量，用来记录每个字母上一次出现的位置
        let mut last_index = vec![None; 26];
        // 遍历字符串s中的每个字符和它的索引
        for (i, c) in s.chars().enumerate() {
            // 计算字母c在字母表中的索引
            let index = (c as u8 - b'a') as usize;
            // 如果字母c之前出现过
            if let Some(last) = last_index[index] {
                // 判断当前位置和上一次出现位置之间的距离是否等于distance向量中对应字母的距离
                if i as i32 - last - 1 != distance[index] {
                    return false;
                }
            }
            // 更新字母c在last_index向量中的位置
            last_index[index] = Some(i as i32);
        }
        // 如果遍历完整个字符串都没有返回false，则返回true
        true
    }
}