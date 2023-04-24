// 按字典序排在最后的子串
// https://leetcode.cn/problems/last-substring-in-lexicographical-order
// INLINE  ../../images/string/last_substring_in_lexicographical_order.jpeg

pub struct Solution;

impl Solution {
    pub fn last_substring(s: String) -> String {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = 1;
        #[warn(unused_assignments)]
        let mut k: usize = 0;

        while j < n {
            k = 0;
            while j + k < n && chars[i + k] == chars[j + k] {
                k += 1;
            }
            if j + k < n && chars[i + k] < chars[j + k] {
                let t = i;
                i = j;
                j = std::cmp::max(j + 1, t + k + 1);
            } else {
                j = j + k + 1;
            }
        }
        chars[i..].iter().collect()
    }
}
