use rust_practice::stack::valid_parentheses::Solution;

#[test]
fn is_valid() {
    // 示例 1：
    // 输入：s = "()"
    // 输出：true
    assert_eq!(Solution::is_valid("()".to_string()), true);

    // 示例 2：
    // 输入：s = "()[]{}"
    // 输出：true
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);

    // 示例 3：
    // 输入：s = "(]"
    // 输出：false
    assert_eq!(Solution::is_valid("(]".to_string()), false);

    // 示例 4：
    // 输入：s = "([)]"
    // 输出：false
    assert_eq!(Solution::is_valid("([)]".to_string()), false);

    // 示例 5：
    // 输入：s = "{[]}"
    // 输出：true
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
}
