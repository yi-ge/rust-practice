use rust_practice::string::string_rotation_lcci::Solution;

#[test]
fn is_fliped_string() {
    // 示例1:
    //  输入：s1 = "waterbottle", s2 = "erbottlewat"
    //  输出：True
    let s1 = String::from("waterbottle");
    let s2 = String::from("erbottlewat");
    assert!(Solution::is_fliped_string(s1, s2));

    // 示例2:
    //  输入：s1 = "aa", s2 = "aba"
    //  输出：False
    let s1 = String::from("aa");
    let s2 = String::from("aba");
    assert!(!Solution::is_fliped_string(s1, s2));

    assert!(Solution::is_fliped_string("".to_string(), "".to_string()));

    let s1 = String::from("abcd");
    let s2 = String::from("acdb");
    assert!(!Solution::is_fliped_string(s1, s2));
}
