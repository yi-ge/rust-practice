use rust_practice::string::check_if_binary_string_has_at_most_one_segment_of_ones::Solution;

#[test]
fn check_ones_segment() {
    // 示例 1：
    // 输入：s = "1001"
    // 输出：false
    // 解释：由连续若干个 '1' 组成的字段数量为 2，返回 false
    assert!(!Solution::check_ones_segment("1001".to_string()));

    // 示例 2：
    // 输入：s = "110"
    // 输出：true
    assert!(Solution::check_ones_segment("110".to_string()));
}
