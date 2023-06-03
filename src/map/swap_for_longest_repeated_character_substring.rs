// 单字符重复子串的最大长度
// https://leetcode.cn/problems/swap-for-longest-repeated-character-substring
// INLINE  ../../images/map/swap_for_longest_repeated_character_substring.jpeg

// use std::convert::TryInto;
pub struct Solution;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let text = text.chars().collect::<Vec<char>>();
        let (mut l, n, mut ans) = (0, text.len(), 0);
        while l < n {
            let mut r = l + 1;
            while r < n && text[r] == text[l] {
                r += 1;
            } // Find the first location different from text[l]
            if r == n {
                return ans.max(
                    (r - l + if text[0..l].contains(&text[l]) { 1 } else { 0 })
                        .try_into()
                        .unwrap(),
                );
            } // Handling at the end of the string
            let l_ = r;
            r += 1;
            while r < n && text[r] == text[l] {
                r += 1;
            } // Find the second location different from text[l]
            if r - l > ans.try_into().unwrap() {
                ans = ans.max(
                    (r - l
                        - if !text[0..l].contains(&text[l]) && !text[r..n].contains(&text[l]) {
                            1
                        } else {
                            0
                        })
                    .try_into()
                    .unwrap(),
                );
            } // Handle
            l = l_;
        }
        ans as i32
    }
}
