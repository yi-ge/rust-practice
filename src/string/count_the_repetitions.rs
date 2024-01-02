// 统计重复个数
// https://leetcode.cn/problems/count-the-repetitions
// INLINE  ../../images/string/count_the_repetitions.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let (len1, len2) = (s1.len(), s2.len());
        let (mut index1, mut index2) = (0, 0);

        if len1 == 0 || len2 == 0 || (len1 * n1 as usize) < (len2 * n2 as usize) {
            return 0;
        }

        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut ans = 0;

        while index1 / len1 < n1 as usize {
            if index1 % len1 == len1 - 1 {
                let key = index2 % len2;
                if let Some(&val) = map1.get(&key) {
                    let cycle_len = index1 / len1 - val / len1;
                    let cycle_num = (n1 as usize - 1 - index1 / len1) / cycle_len;
                    let cycle_s2_num = index2 / len2 - map2[&key] / len2;

                    index1 += cycle_num * cycle_len * len1;
                    ans += cycle_num * cycle_s2_num;
                } else {
                    map1.insert(key, index1);
                    map2.insert(key, index2);
                }
            }

            if s1.as_bytes()[index1 % len1] == s2.as_bytes()[index2 % len2] {
                if index2 % len2 == len2 - 1 {
                    ans += 1;
                }
                index2 += 1;
            }
            index1 += 1;
        }

        ans as i32 / n2
    }
}
