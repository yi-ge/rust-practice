// 删除字符使频率相同
// https://leetcode.cn/problems/remove-letter-to-equalize-frequency
// INLINE  ../../images/map/remove_letter_to_equalize_frequency.jpeg

pub struct Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        // 建立字符与出现次数的映射
        let mut map = std::collections::HashMap::new();
        for c in word.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        // 将出现次数按升序排列
        let mut values: Vec<i32> = map.values().cloned().collect();
        values.sort();
        // 如果所有字符出现次数都相同，则无需删除字符
        if values.len() == 1 {
            #[cfg_attr(tarpaulin, skip)]
            return true;
        }
        // 如果只需删除一个字符即可使所有字符出现次数相同，则返回 true
        if values[0] == 1 && values[1] == values[values.len() - 1] {
            #[cfg_attr(tarpaulin, skip)]
            return true;
        }
        // 如果只需删除一个字符即可使最大出现次数与次大出现次数相同，并且所有字符出现次数相同，则返回 true
        if values[values.len() - 1] - values[values.len() - 2] == 1
            && values[0] == values[values.len() - 2]
        {
            return true;
        }
        false
    }
}
