use rust_practice::array::find_maximum_number_of_string_pairs::Solution;

#[test]
fn maximum_number_of_string_pairs_test() {
    // 示例 1：
    // 输入：words = ["cd","ac","dc","ca","zz"]
    // 输出：2
    // 解释：在此示例中，我们可以通过以下方式匹配 2 对字符串：
    // - 我们将第 0 个字符串与第 2 个字符串匹配，因为 word[0] 的反转字符串是 "dc" 并且等于 words[2]。
    // - 我们将第 1 个字符串与第 3 个字符串匹配，因为 word[1] 的反转字符串是 "ca" 并且等于 words[3]。
    // 可以证明最多匹配数目是 2 。
    let words = vec![
        "cd".to_string(),
        "ac".to_string(),
        "dc".to_string(),
        "ca".to_string(),
        "zz".to_string(),
    ];
    assert_eq!(Solution::maximum_number_of_string_pairs(words), 2);

    // 示例 2：
    // 输入：words = ["ab","ba","cc"]
    // 输出：1
    // 解释：在此示例中，我们可以通过以下方式匹配 1 对字符串：
    // - 我们将第 0 个字符串与第 1 个字符串匹配，因为 words[1] 的反转字符串 "ab" 与 words[0] 相等。
    // 可以证明最多匹配数目是 1 。
    let words = vec!["ab".to_string(), "ba".to_string(), "cc".to_string()];
    assert_eq!(Solution::maximum_number_of_string_pairs(words), 1);

    // 示例 3：
    // 输入：words = ["aa","ab"]
    // 输出：0
    // 解释：这个例子中，无法匹配任何字符串。
    let words = vec!["aa".to_string(), "ab".to_string()];
    assert_eq!(Solution::maximum_number_of_string_pairs(words), 0);
}
