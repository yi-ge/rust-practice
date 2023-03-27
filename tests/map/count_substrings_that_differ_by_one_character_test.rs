use rust_practice::map::count_substrings_that_differ_by_one_character::Solution;

#[test]
fn count_substrings() {
    // 示例 1：
    // 输入：s = "aba", t = "baba"
    // 输出：6
    // 解释：以下为只相差 1 个字符的 s 和 t 串的子字符串对：
    // ("aba", "baba")
    // ("aba", "baba")
    // ("aba", "baba")
    // ("aba", "baba")
    // ("aba", "baba")
    // ("aba", "baba")
    // 加粗部分分别表示 s 和 t 串选出来的子字符串。
    assert_eq!(
        Solution::count_substrings("aba".to_string(), "baba".to_string()),
        6
    );

    // 示例 2：
    // 输入：s = "ab", t = "bb"
    // 输出：3
    // 解释：以下为只相差 1 个字符的 s 和 t 串的子字符串对：
    // ("ab", "bb")
    // ("ab", "bb")
    // ("ab", "bb")
    // 加粗部分分别表示 s 和 t 串选出来的子字符串。
    assert_eq!(
        Solution::count_substrings("ab".to_string(), "bb".to_string()),
        3
    );

    // 示例 3：
    // 输入：s = "a", t = "a"
    // 输出：0
    assert_eq!(
        Solution::count_substrings("a".to_string(), "a".to_string()),
        0
    );

    // 示例 4：
    // 输入：s = "abe", t = "bbc"
    // 输出：10
    assert_eq!(
        Solution::count_substrings("abe".to_string(), "bbc".to_string()),
        10
    );
}
