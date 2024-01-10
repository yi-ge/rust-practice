use rust_practice::stack::minimum_string_length_after_removing_substrings::Solution;

#[test]
fn min_length() {
    // 示例 1：
    // 输入：s = "ABFCACDB"
    // 输出：2
    // 解释：你可以执行下述操作：
    // - 从 "ABFCACDB" 中删除子串 "AB"，得到 s = "FCACDB" 。
    // - 从 "FCACDB" 中删除子串 "CD"，得到 s = "FCAB" 。
    // - 从 "FCAB" 中删除子串 "AB"，得到 s = "FC" 。
    // 最终字符串的长度为 2 。
    // 可以证明 2 是可获得的最小长度。
    let s = "ABFCACDB".to_string();
    assert_eq!(Solution::min_length(s), 2);

    // 示例 2：
    // 输入：s = "ACBBD"
    // 输出：5
    // 解释：无法执行操作，字符串长度不变。
    let s = "ACBBD".to_string();
    assert_eq!(Solution::min_length(s), 5);
}
