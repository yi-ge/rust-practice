use rust_practice::math::count_sorted_vowel_strings::Solution;

#[test]
fn test_count_vowel_strings() {
    // 示例 1：
    // 输入：n = 1
    // 输出：5
    // 解释：仅由元音组成的 5 个字典序字符串为 ["a","e","i","o","u"]
    assert_eq!(Solution::count_vowel_strings(1), 5);

    // 示例 2：
    // 输入：n = 2
    // 输出：15
    // 解释：仅由元音组成的 15 个字典序字符串为
    // ["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"]
    // 注意，"ea" 不是符合题意的字符串，因为 'e' 在字母表中的位置比 'a' 靠后
    assert_eq!(Solution::count_vowel_strings(2), 15);

    // 示例 3：
    // 输入：n = 33
    // 输出：66045
    assert_eq!(Solution::count_vowel_strings(33), 66045);
}
