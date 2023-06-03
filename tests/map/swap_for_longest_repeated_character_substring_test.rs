use rust_practice::map::swap_for_longest_repeated_character_substring::Solution;

#[test]
fn max_rep_opt1() {
    // 示例 1：
    // 输入：text = "ababa"
    // 输出：3
    let text = "ababa".to_string();
    assert_eq!(Solution::max_rep_opt1(text), 3);

    // 示例 2：
    // 输入：text = "aaabaaa"
    // 输出：6
    let text = "aaabaaa".to_string();
    assert_eq!(Solution::max_rep_opt1(text), 6);

    // 示例 3：
    // 输入：text = "aaabbaaa"
    // 输出：4
    let text = "aaabbaaa".to_string();
    assert_eq!(Solution::max_rep_opt1(text), 4);

    // 示例 4：
    // 输入：text = "aaaaa"
    // 输出：5
    let text = "aaaaa".to_string();
    assert_eq!(Solution::max_rep_opt1(text), 5);

    // 示例 5：
    // 输入：text = "abcdef"
    // 输出：1
    let text = "abcdef".to_string();
    assert_eq!(Solution::max_rep_opt1(text), 1);
}
