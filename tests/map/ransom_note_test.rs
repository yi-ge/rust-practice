use rust_practice::map::ransom_note::Solution;

#[test]
fn can_construct() {
    // 示例 1：
    // 输入：ransomNote = "a", magazine = "b"
    // 输出：false
    let ransom_note = "a".to_string();
    let magazine = "b".to_string();
    assert_eq!(Solution::can_construct(ransom_note, magazine), false);

    // 示例 2：
    // 输入：ransomNote = "aa", magazine = "ab"
    // 输出：false
    let ransom_note = "aa".to_string();
    let magazine = "ab".to_string();
    assert_eq!(Solution::can_construct(ransom_note, magazine), false);

    // 示例 3：
    // 输入：ransomNote = "aa", magazine = "aab"
    // 输出：true
    let ransom_note = "aa".to_string();
    let magazine = "aab".to_string();
    assert_eq!(Solution::can_construct(ransom_note, magazine), true);
}
