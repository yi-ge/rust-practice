use rust_practice::string::construct_string_with_repeat_limit::Solution;

#[test]
fn repeat_limited_string() {
    // 示例 1：
    // 输入：s = "cczazcc", repeatLimit = 3
    // 输出："zzcccac"
    // 解释：使用 s 中的所有字符来构造 repeatLimitedString "zzcccac"。
    // 字母 'a' 连续出现至多 1 次。
    // 字母 'c' 连续出现至多 3 次。
    // 字母 'z' 连续出现至多 2 次。
    // 因此，没有字母连续出现超过 repeatLimit 次，字符串是一个有效的 repeatLimitedString 。
    // 该字符串是字典序最大的 repeatLimitedString ，所以返回 "zzcccac" 。
    // 注意，尽管 "zzcccca" 字典序更大，但字母 'c' 连续出现超过 3 次，所以它不是一个有效的 repeatLimitedString 。
    let s = "cczazcc".to_string();
    let repeat_limit = 3;
    assert_eq!(
        Solution::repeat_limited_string(s, repeat_limit),
        "zzcccac".to_string()
    );

    // 示例 2：
    // 输入：s = "aababab", repeatLimit = 2
    // 输出："bbabaa"
    // 解释：
    // 使用 s 中的一些字符来构造 repeatLimitedString "bbabaa"。
    // 字母 'a' 连续出现至多 2 次。
    // 字母 'b' 连续出现至多 2 次。
    // 因此，没有字母连续出现超过 repeatLimit 次，字符串是一个有效的 repeatLimitedString 。
    // 该字符串是字典序最大的 repeatLimitedString ，所以返回 "bbabaa" 。
    // 注意，尽管 "bbabaaa" 字典序更大，但字母 'a' 连续出现超过 2 次，所以它不是一个有效的 repeatLimitedString 。
    let s = "aababab".to_string();
    let repeat_limit = 2;
    assert_eq!(
        Solution::repeat_limited_string(s, repeat_limit),
        "bbabaa".to_string()
    );
}
