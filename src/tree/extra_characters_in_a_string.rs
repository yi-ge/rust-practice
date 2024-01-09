use std::collections::HashMap;
// 字符串中的额外字符
// https://leetcode.cn/problems/extra-characters-in-a-string
// INLINE  ../../images/tree/extra_characters_in_a_string.jpeg

pub struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let mut d = vec![std::i32::MAX; n + 1];
        let mut mp: HashMap<String, i32> = HashMap::new();

        for word in dictionary {
            *mp.entry(word).or_insert(0) += 1;
        }

        d[0] = 0;

        for i in 1..=n {
            d[i] = d[i - 1] + 1;

            for j in (0..i).rev() {
                if mp.contains_key(&s[j..i]) {
                    d[i] = d[i].min(d[j]);
                }
            }
        }

        d[n]
    }
}
