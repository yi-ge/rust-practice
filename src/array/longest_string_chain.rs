// 最长字符串链
// https://leetcode.cn/problems/longest-string-chain
// INLINE  ../../images/array/longest_string_chain.jpeg

pub struct Solution;

impl Solution {
    fn is_predecessor(a: &str, b: &str) -> bool {
        if a.len() + 1 != b.len() {
            return false;
        }
        let mut i = 0;
        let mut j = 0;
        let mut diff = 0;
        while i < a.len() && j < b.len() {
            if a.chars().nth(i) == b.chars().nth(j) {
                i += 1;
                j += 1;
            } else {
                diff += 1;
                j += 1;
            }
        }
        diff <= 1
    }
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut dp = vec![1; words.len()];
        let mut max = 1;
        for i in 1..words.len() {
            for j in 0..i {
                if Solution::is_predecessor(&words[j], &words[i]) {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max = max.max(dp[i]);
        }
        max
    }
}
