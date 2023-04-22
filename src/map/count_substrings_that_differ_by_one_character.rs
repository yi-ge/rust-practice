// 统计只差一个字符的子串数目
// https://leetcode.cn/problems/count-substrings-that-differ-by-one-character
// INLINE  ../../images/map/count_substrings_that_differ_by_one_character.jpeg

pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let m = s.len();
        let n = t.len();
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();

        let mut dpl = vec![vec![0; n + 1]; m + 1];
        let mut dpr = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                dpl[i + 1][j + 1] = if s[i] == t[j] { dpl[i][j] + 1 } else { 0 };
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                dpr[i][j] = if s[i] == t[j] {
                    dpr[i + 1][j + 1] + 1
                } else {
                    0
                };
            }
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if s[i] != t[j] {
                    ans += (dpl[i][j] + 1) * (dpr[i + 1][j + 1] + 1);
                }
            }
        }

        ans as i32
    }
}
