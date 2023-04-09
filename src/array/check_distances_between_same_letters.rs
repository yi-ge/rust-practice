// 检查相同字母间的距离
// https://leetcode.cn/problems/check-distances-between-same-letters
// INLINE  ../../images/array/check_distances_between_same_letters.jpeg

pub struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut last_index = vec![None; 26];
        for (i, c) in s.chars().enumerate() {
            let index = (c as u8 - b'a') as usize;
            if let Some(last) = last_index[index] {
                if i as i32 - last - 1 < distance[index] {
                    return false;
                }
            }
            last_index[index] = Some(i as i32);
        }
        true
    }
}
