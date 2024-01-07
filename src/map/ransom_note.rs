// 赎金信
// https://leetcode.cn/problems/ransom-note
// INLINE  ../../images/map/ransom_note.jpeg

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut count: [i32; 26] = [0; 26];
        for c in magazine.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        for c in ransom_note.chars() {
            count[(c as u8 - b'a') as usize] -= 1;
            if count[(c as u8 - b'a') as usize] < 0 {
                return false;
            }
        }

        true
    }
}
