// 最短公共超序列
// https://leetcode.cn/problems/shortest-common-supersequence
// INLINE  ../../images/string/shortest_common_supersequence.jpeg

pub struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let n = str1.len();
        let m = str2.len();
        let mut f = vec![vec![0; m + 1]; n + 1];

        for j in 0..=m {
            f[0][j] = j;
        }
        for i in 1..=n {
            f[i][0] = i;
        }
        for i in 1..=n {
            for j in 1..=m {
                if str1.bytes().nth(i - 1) == str2.bytes().nth(j - 1) {
                    f[i][j] = f[i - 1][j - 1] + 1;
                } else {
                    f[i][j] = std::cmp::min(f[i][j - 1], f[i - 1][j]) + 1;
                }
            }
        }

        let mut ans = String::new();
        let mut i = n;
        let mut j = m;

        let mut append_char = |idx: usize, s: &String| {
            ans.insert(0, s.bytes().nth(idx - 1).unwrap() as char);
        };

        while i > 0 && j > 0 {
            if str1.bytes().nth(i - 1) == str2.bytes().nth(j - 1) {
                append_char(i, &str1);
                i -= 1;
                j -= 1;
            } else if f[i][j - 1] < f[i - 1][j] {
                append_char(j, &str2);
                j -= 1;
            } else {
                append_char(i, &str1);
                i -= 1;
            }
        }
        while i > 0 {
            append_char(i, &str1);
            i -= 1;
        }
        while j > 0 {
            append_char(j, &str2);
            j -= 1;
        }

        ans
    }
}
