// 删除字符使频率相同
// https://leetcode.cn/problems/remove-letter-to-equalize-frequency
// INLINE  ../../images/map/remove_letter_to_equalize_frequency.jpeg

pub struct Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut map = std::collections::HashMap::new();
        for c in word.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut values: Vec<i32> = map.values().cloned().collect();
        values.sort();
        if values.len() == 1 {
            return true;
        }
        if values[0] == 1 && values[1] == values[values.len() - 1] {
            return true;
        }
        if values[values.len() - 1] - values[values.len() - 2] == 1
            && values[0] == values[values.len() - 2]
        {
            return true;
        }
        false
    }
}
