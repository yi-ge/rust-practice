use rust_practice::array::split_strings_by_separator::Solution;

#[test]
fn split_words_by_separator_test() {
    // 示例 1：
    // 输入：words = ["one.two.three","four.five","six"], separator = "."
    // 输出：["one","two","three","four","five","six"]
    // 解释：在本示例中，我们进行下述拆分：

    // "one.two.three" 拆分为 "one", "two", "three"
    // "four.five" 拆分为 "four", "five"
    // "six" 拆分为 "six"

    // 因此，结果数组为 ["one","two","three","four","five","six"] 。
    let words = vec![
        "one.two.three".to_string(),
        "four.five".to_string(),
        "six".to_string(),
    ];
    let separator = '.';
    let result = Solution::split_words_by_separator(words, separator);
    assert_eq!(
        result,
        vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
        ]
    );

    // 示例 2：
    // 输入：words = ["$easy$","$problem$"], separator = "$"
    // 输出：["easy","problem"]
    // 解释：在本示例中，我们进行下述拆分：

    // "$easy$" 拆分为 "easy"（不包括空字符串）
    // "$problem$" 拆分为 "problem"（不包括空字符串）

    // 因此，结果数组为 ["easy","problem"] 。
    let words = vec!["$easy$".to_string(), "$problem$".to_string()];
    let separator = '$';
    let result = Solution::split_words_by_separator(words, separator);
    assert_eq!(result, vec!["easy".to_string(), "problem".to_string()]);

    // 示例 3：
    // 输入：words = ["|||"], separator = "|"
    // 输出：[]
    // 解释：在本示例中，"|||" 的拆分结果将只包含一些空字符串，所以我们返回一个空数组 [] 。
    let words = vec!["|||".to_string()];
    let separator = '|';
    let result = Solution::split_words_by_separator(words, separator);
    assert_eq!(result.len(), 0);
}
