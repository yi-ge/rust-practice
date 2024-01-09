use rust_practice::tree::extra_characters_in_a_string::Solution;

#[test]
fn min_extra_char() {
    // 示例 1：
    // 输入：s = "leetscode", dictionary = ["leet","code","leetcode"]
    // 输出：1
    // 解释：将 s 分成两个子字符串：下标从 0 到 3 的 "leet" 和下标从 5 到 8 的 "code" 。只有 1 个字符没有使用（下标为 4），所以我们返回 1 。
    let s = "leetscode".to_string();
    let dictionary = vec![
        "leet".to_string(),
        "code".to_string(),
        "leetcode".to_string(),
    ];
    assert_eq!(Solution::min_extra_char(s, dictionary), 1);

    // 示例 2：
    // 输入：s = "sayhelloworld", dictionary = ["hello","world"]
    // 输出：3
    // 解释：将 s 分成两个子字符串：下标从 3 到 7 的 "hello" 和下标从 8 到 12 的 "world" 。下标为 0 ，1 和 2 的字符没有使用，所以我们返回 3 。
    let s = "sayhelloworld".to_string();
    let dictionary = vec!["hello".to_string(), "world".to_string()];
    assert_eq!(Solution::min_extra_char(s, dictionary), 3);
}
