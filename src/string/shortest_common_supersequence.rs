// 最短公共超序列
// https://leetcode.cn/problems/shortest-common-supersequence
// INLINE  ../../images/string/shortest_common_supersequence.jpeg

pub struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let n = str1.len(); // 字符串1的长度
        let m = str2.len(); // 字符串2的长度
        let mut f = vec![vec![0; m + 1]; n + 1]; // 初始化二维数组f，用于动态规划

        for j in 0..=m {
            // 初始化f的第0行
            f[0][j] = j;
        }
        for i in 1..=n {
            // 初始化f的第0列
            f[i][0] = i;
        }
        for i in 1..=n {
            // 动态规划求解f
            for j in 1..=m {
                if str1.bytes().nth(i - 1) == str2.bytes().nth(j - 1) {
                    // 如果两个字符相同
                    f[i][j] = f[i - 1][j - 1] + 1; // 则f[i][j]等于左上角的值加1
                } else {
                    // 如果两个字符不同
                    f[i][j] = std::cmp::min(f[i][j - 1], f[i - 1][j]) + 1; // 则f[i][j]等于左边和上边的值的最小值加1
                }
            }
        }

        let mut ans = String::new(); // 初始化最短公共超序列的字符串
        let mut i = n; // 从字符串1的最后一个字符开始
        let mut j = m; // 从字符串2的最后一个字符开始

        let mut append_char = |idx: usize, s: &String| {
            // 定义一个闭包，用于向最短公共超序列中添加字符
            ans.insert(0, s.bytes().nth(idx - 1).unwrap() as char); // 将字符插入到最短公共超序列的前面，保证最终的顺序是正确的
        };

        while i > 0 && j > 0 {
            // 从后往前遍历二维数组f，将字符添加到最短公共超序列中
            if str1.bytes().nth(i - 1) == str2.bytes().nth(j - 1) {
                // 如果两个字符相同
                append_char(i, &str1); // 则将该字符添加到最短公共超序列中
                i -= 1; // 继续向前遍历
                j -= 1;
            } else if f[i][j - 1] < f[i - 1][j] {
                // 如果f[i][j - 1]小于f[i - 1][j]
                append_char(j, &str2); // 则将字符串2的当前字符添加到最短公共超序列中
                j -= 1; // 继续向前遍历
            } else {
                append_char(i, &str1); // 否则将字符串1的当前字符添加到最短公共超序列中
                i -= 1; // 继续向前遍历
            }
        }
        while i > 0 {
            // 如果字符串2已经遍历完了，但是字符串1还有剩余字符
            append_char(i, &str1); // 则将字符串1的当前字符添加到最短公共超序列中
            i -= 1; // 继续向前遍历
        }
        while j > 0 {
            // 如果字符串1已经遍历完了，但是字符串2还有剩余字符
            append_char(j, &str2); // 则将字符串2的当前字符添加到最短公共超序列中
            j -= 1; // 继续向前遍历
        }

        ans // 返回最短公共超序列
    }
}
