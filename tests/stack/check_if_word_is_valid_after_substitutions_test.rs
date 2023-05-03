use rust_practice::stack::check_if_word_is_valid_after_substitutions::Solution;

#[test]
fn is_valid() {
    // 示例 1：
    // 输入：s = "aabcbc"
    // 输出：true
    // 解释：
    // "" -> "abc" -> "aabcbc"
    // 因此，"aabcbc" 有效。
    let s = "aabcbc".to_string();
    assert_eq!(Solution::is_valid(s), true);

    // 示例 2：
    // 输入：s = "abcabcab。abcc"
    // 输出：true
    // 解释：
    // "" -> "abc" -> "abcabc" -> "abcabcabc" -> "abcabcababcc"
    // 因此，"abcabcababcc" 有效。
    let s = "abcabcababcc".to_string();
    assert_eq!(Solution::is_valid(s), true);

    // 示例 3：
    // 输入：s = "abccba"
    // 输出：false
    // 解释：执行操作无法得到 "abccba" 。
    let s = "abccba".to_string();
    assert_eq!(Solution::is_valid(s), false);

    // 示例 4：
    // 输入：s = "abcacb"
    // 输出：false
    // 解释：执行操作无法得到 "abcacb" 。
    let s = "abcacb".to_string();
    assert_eq!(Solution::is_valid(s), false);

    // 示例 5：
    // 输入：s = "abacbc"
    // 输出：false
    // 解释：执行操作无法得到 "abacbc" 。
    let s = "abacbc".to_string();
    assert_eq!(Solution::is_valid(s), false);
}
