// 统计只差一个字符的子串数目
// https://leetcode.cn/problems/count-substrings-that-differ-by-one-character
// INLINE  ../../images/map/count_substrings_that_differ_by_one_character.jpeg

pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let m = s.len(); // 获取字符串 s 的长度
        let n = t.len(); // 获取字符串 t 的长度
        let s = s.chars().collect::<Vec<_>>(); // 将字符串 s 转换为字符数组
        let t = t.chars().collect::<Vec<_>>(); // 将字符串 t 转换为字符数组

        let mut dpl = vec![vec![0; n + 1]; m + 1]; // 初始化 dpl 数组
        let mut dpr = vec![vec![0; n + 1]; m + 1]; // 初始化 dpr 数组

        for i in 0..m {
            for j in 0..n {
                // 如果 s[i] 和 t[j] 相等，则 dpl[i + 1][j + 1] 的值为 dpl[i][j] + 1，否则为 0
                dpl[i + 1][j + 1] = if s[i] == t[j] { dpl[i][j] + 1 } else { 0 };
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                // 如果 s[i] 和 t[j] 相等，则 dpr[i][j] 的值为 dpr[i + 1][j + 1] + 1，否则为 0
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
                    // 如果 s[i] 和 t[j] 不相等
                    ans += (dpl[i][j] + 1) * (dpr[i + 1][j + 1] + 1); // 将 dpl[i][j] + 1 和 dpr[i + 1][j + 1] + 1 的乘积加到 ans 中
                }
            }
        }

        ans as i32 // 将 ans 转换为 i32 类型并返回
    }
}
