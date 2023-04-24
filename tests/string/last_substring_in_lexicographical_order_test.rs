use rust_practice::string::last_substring_in_lexicographical_order::Solution;

#[test]
fn last_substring() {
    // 示例 1：
    // 输入：s = "abab"
    // 输出："bab"
    // 解释：我们可以找出 7 个子串 ["a", "ab", "aba", "abab", "b", "ba", "bab"]。按字典序排在最后的子串是 "bab"。
    let s = "abab".to_string();
    assert_eq!(Solution::last_substring(s), "bab".to_string());

    // 示例 2：
    // 输入：s = "leetcode"
    // 输出："tcode"
    let s = "leetcode".to_string();
    assert_eq!(Solution::last_substring(s), "tcode".to_string());
}
