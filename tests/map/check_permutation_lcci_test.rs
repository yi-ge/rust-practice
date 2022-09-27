use rust_practice::map::check_permutation_lcci::Solution;

#[test]
fn check_permutation() {
    // 示例 1：
    // 输入: s1 = "abc", s2 = "bca"
    // 输出: true
    assert!(Solution::check_permutation(
        "abc".to_string(),
        "bca".to_string()
    ));

    // 示例 2：
    // 输入: s1 = "abc", s2 = "bad"
    // 输出: false
    assert_eq!(
        Solution::check_permutation("abc".to_string(), "bad".to_string()),
        false
    );

    assert_eq!(
        Solution::check_permutation("abc".to_string(), "d".to_string()),
        false
    );
}
