// 最长字符串链
// https://leetcode.cn/problems/longest-string-chain
// INLINE  ../../images/array/longest_string_chain.jpeg

pub struct Solution;

impl Solution {
    // 判断字符串a是否是字符串b的前驱
    fn is_predecessor(a: &str, b: &str) -> bool {
        // 如果a长度加1不等于b长度，a一定不是b的前驱
        if a.len() + 1 != b.len() {
            return false;
        }
        let mut i = 0;
        let mut j = 0;
        let mut diff = 0;
        // 遍历a和b的字符，如果不同，只能让j加一，即跳过b中的当前字符，让a和b后面的字符比较
        while i < a.len() && j < b.len() {
            if a.chars().nth(i) == b.chars().nth(j) {
                i += 1;
                j += 1;
            } else {
                diff += 1;
                j += 1;
            }
        }
        // 如果不同字符个数小于等于1，说明a是b的前驱
        diff <= 1
    }

    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        // 按字符串长度从小到大排序
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut dp = vec![1; words.len()];
        let mut max = 1;
        for i in 1..words.len() {
            for j in 0..i {
                if Solution::is_predecessor(&words[j], &words[i]) {
                    // 如果words[j]是words[i]的前驱，更新dp[i]
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            // 更新最长的字符串链长度
            max = max.max(dp[i]);
        }
        max
    }
}
