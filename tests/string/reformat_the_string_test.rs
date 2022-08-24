use rust_practice::string::reformat_the_string::Solution;

// https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html
#[test]
fn reformat() {
    // 示例 1：
    // 输入：s = "a0b1c2"
    // 输出："0a1b2c"
    // 解释："0a1b2c" 中任意两个相邻字符的类型都不同。 "a0b1c2", "0a1b2c", "0c2a1b" 也是满足题目要求的答案。
    assert_eq!(Solution::reformat("a0b1c2".to_string()), "a0b1c2");

    // 示例 2：
    // 输入：s = "leetcode"
    // 输出：""
    // 解释："leetcode" 中只有字母，所以无法满足重新格式化的条件。
    assert_eq!(Solution::reformat("leetcode".to_string()), "");

    // 示例 3：
    // 输入：s = "1229857369"
    // 输出：""
    // 解释："1229857369" 中只有数字，所以无法满足重新格式化的条件。
    assert_eq!(Solution::reformat("1229857369".to_string()), "");

    // 示例 4：
    // 输入：s = "covid2019"
    // 输出："c2o0v1i9d"
    assert_eq!(Solution::reformat("covid2019".to_string()), "c2o0v1i9d");

    // 示例 5：
    // 输入：s = "ab123"
    // 输出："1a2b3"
    assert_eq!(Solution::reformat("ab123".to_string()), "1a2b3")
}