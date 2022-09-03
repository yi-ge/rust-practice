use rust_practice::map::valid_anagram::Solution;

#[test]
fn is_anagram() {
    // 示例 1:
    // 输入: s = "anagram", t = "nagaram"
    // 输出: true
    let s = String::from("anagram");
    let t = String::from("nagaram");
    assert_eq!(Solution::is_anagram(s, t), true);

    // 示例 2:
    // 输入: s = "rat", t = "car"
    // 输出: false
    let s = String::from("rat");
    let t = String::from("car");
    assert_eq!(Solution::is_anagram(s, t), false);

    // 示例 3:
    // 输入: s = "rat", t = "ca"
    // 输出: false
    let s = String::from("rat");
    let t = String::from("ca");
    assert_eq!(Solution::is_anagram(s, t), false);
}
