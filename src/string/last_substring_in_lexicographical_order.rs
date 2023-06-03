// 按字典序排在最后的子串
// https://leetcode.cn/problems/last-substring-in-lexicographical-order
// INLINE  ../../images/string/last_substring_in_lexicographical_order.jpeg

pub struct Solution;

impl Solution {
    pub fn last_substring(s: String) -> String {
        let n = s.len(); // 字符串s的长度
        let chars: Vec<char> = s.chars().collect(); // 将字符串s转换为字符数组
        let mut i = 0; // 左指针
        let mut j = 1; // 右指针
        #[allow(unused_assignments)]
        let mut k: usize = 0; // 相同字符的数量

        while j < n {
            // 右指针不断向右移动
            k = 0; // 每次开始比较前需要将k清零
            while j + k < n && chars[i + k] == chars[j + k] {
                // 比较i+k和j+k位置上的字符是否相同
                k += 1; // 如果相同，k+1
            }
            if j + k < n && chars[i + k] < chars[j + k] {
                // 如果i+k位置上的字符小于j+k位置上的字符
                let t = i; // 交换i和j的位置
                i = j;
                j = std::cmp::max(j + 1, t + k + 1); // j向右移动，确保j和i没有重叠部分
            } else {
                // 如果i+k位置上的字符大于或等于j+k位置上的字符
                j = j + k + 1; // j向右移动k+1个位置
            }
        }
        chars[i..].iter().collect() // 返回从i开始的子串
    }
}
