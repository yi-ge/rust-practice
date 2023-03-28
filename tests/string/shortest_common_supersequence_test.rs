use rust_practice::string::shortest_common_supersequence::Solution;

#[test]
fn shortest_common_supersequence() {
    // 示例：
    // 输入：str1 = "abac", str2 = "cab"
    // 输出："cabac"
    // 解释：
    // str1 = "abac" 是 "cabac" 的一个子串，因为我们可以删去 "cabac" 的第一个 "c"得到 "abac"。
    // str2 = "cab" 是 "cabac" 的一个子串，因为我们可以删去 "cabac" 末尾的 "ac" 得到 "cab"。
    // 最终我们给出的答案是满足上述属性的最短字符串。

    assert_eq!(
        Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string()),
        "cabac".to_string()
    );

    // Test with empty strings:
    // Input: str1 = "", str2 = ""
    // Output: ""
    assert_eq!(
        Solution::shortest_common_supersequence("".to_string(), "".to_string()),
        "".to_string()
    );

    // Test with one empty string:
    // Input: str1 = "abc", str2 = ""
    // Output: "abc"
    assert_eq!(
        Solution::shortest_common_supersequence("abc".to_string(), "".to_string()),
        "abc".to_string()
    );

    // Test with identical strings:
    // Input: str1 = "abc", str2 = "abc"
    // Output: "abc"
    assert_eq!(
        Solution::shortest_common_supersequence("abc".to_string(), "abc".to_string()),
        "abc".to_string()
    );

    // Test with no common characters:
    // Input: str1 = "abc", str2 = "def"
    // Output: "defabc"
    assert_eq!(
        Solution::shortest_common_supersequence("abc".to_string(), "def".to_string()),
        "defabc".to_string()
    );

    // Test with partially overlapping strings:
    // Input: str1 = "xyabc", str2 = "defabc"
    // Output: "defxyabc"
    assert_eq!(
        Solution::shortest_common_supersequence("xyabc".to_string(), "defabc".to_string()),
        "defxyabc".to_string()
    );

    // Test case that covers the missing branch:
    // Input: str1 = "abcde", str2 = "zabcf"
    // Output: "zabcfde"
    assert_eq!(
        Solution::shortest_common_supersequence("abcde".to_string(), "zabcf".to_string()),
        "zabcfde".to_string()
    );
}
