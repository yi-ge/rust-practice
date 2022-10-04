use rust_practice::stack::minimum_add_to_make_parentheses_valid::Solution;

#[test]
fn min_add_to_make_valid() {
    // 示例 1：
    // 输入：s = "())"
    // 输出：1
    assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);

    // 示例 2：
    // 输入：s = "((("
    // 输出：3
    assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
}
