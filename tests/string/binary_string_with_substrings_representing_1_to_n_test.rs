use rust_practice::string::binary_string_with_substrings_representing_1_to_n::Solution;

#[test]
fn query_string() {
    // 示例 1：
    // 输入：s = "0110", n = 3
    // 输出：true
    let s = "0110".to_string();
    let n = 3;
    assert_eq!(Solution::query_string(s, n), true);

    // 示例 2：
    // 输入：s = "0110", n = 4
    // 输出：false
    let s = "0110".to_string();
    let n = 4;
    assert_eq!(Solution::query_string(s, n), false);
}
