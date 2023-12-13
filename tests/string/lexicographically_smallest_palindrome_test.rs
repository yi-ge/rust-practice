use rust_practice::string::lexicographically_smallest_palindrome::Solution;

#[test]
fn make_smallest_palindrome() {
    // 示例 1：
    // 输入：s = "egcfe"
    // 输出："efcfe"
    // 解释：将 "egcfe" 变成回文字符串的最小操作次数为 1 ，修改 1 次得到的字典序最小回文字符串是 "efcfe"，只需将 'g' 改为 'f' 。
    let s = "egcfe".to_string();
    assert_eq!(Solution::make_smallest_palindrome(s), "efcfe".to_string());

    // 示例 2：
    // 输入：s = "abcd"
    // 输出："abba"
    // 解释：将 "abcd" 变成回文字符串的最小操作次数为 2 ，修改 2 次得到的字典序最小回文字符串是 "abba" 。
    let s = "abcd".to_string();
    assert_eq!(Solution::make_smallest_palindrome(s), "abba".to_string());

    // 示例 3：
    // 输入：s = "seven"
    // 输出："neven"
    // 解释：将 "seven" 变成回文字符串的最小操作次数为 1 ，修改 1 次得到的字典序最小回文字符串是 "neven" 。
    let s = "seven".to_string();
    assert_eq!(Solution::make_smallest_palindrome(s), "neven".to_string());
}
