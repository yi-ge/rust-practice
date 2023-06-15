// 构建回文串检测
// https://leetcode.cn/problems/can-make-palindrome-from-substring
// INLINE  ../../images/math/can_make_palindrome_from_substring.jpeg

pub struct Solution;

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut result = vec![];
        let mut dp = vec![vec![0; 26]];
        for c in s.chars() {
            let mut last = dp.last().unwrap().clone();
            last[c as usize - 'a' as usize] += 1;
            dp.push(last);
        }
        for query in queries {
            let mut count = 0;
            for i in 0..26 {
                count += (dp[query[1] as usize + 1][i] - dp[query[0] as usize][i]) % 2;
            }
            result.push(count / 2 <= query[2]);
        }
        result
    }
}
