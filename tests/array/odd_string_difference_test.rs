use rust_practice::array::odd_string_difference::Solution;

#[test]
fn odd_string() {
    // 示例 1：
    // 输入：words = ["adc","wzy","abc"]
    // 输出："abc"
    // 解释：
    // - "adc" 的差值整数数组是 [3 - 0, 2 - 3] = [3, -1] 。
    // - "wzy" 的差值整数数组是 [25 - 22, 24 - 25]= [3, -1] 。
    // - "abc" 的差值整数数组是 [1 - 0, 2 - 1] = [1, 1] 。
    // 不同的数组是 [1, 1]，所以返回对应的字符串，"abc"。
    let words = vec!["adc".to_string(), "wzy".to_string(), "abc".to_string()];
    let result = Solution::odd_string(words);
    assert_eq!(result, "abc".to_string());

    // 示例 2：
    // 输入：words = ["aaa","bob","ccc","ddd"]
    // 输出："bob"
    // 解释：除了 "bob" 的差值整数数组是 [13, -13] 以外，其他字符串的差值整数数组都是 [0, 0] 。
    let words = vec![
        "aaa".to_string(),
        "bob".to_string(),
        "ccc".to_string(),
        "ddd".to_string(),
    ];
    let result = Solution::odd_string(words);
    assert_eq!(result, "bob".to_string());
}
