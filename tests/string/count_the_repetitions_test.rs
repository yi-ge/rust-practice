use rust_practice::string::count_the_repetitions::Solution;

#[test]
fn get_max_repetitions() {
    // 示例 1：
    // 输入：s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
    // 输出：2
    let s1 = "acb".to_string();
    let n1 = 4;
    let s2 = "ab".to_string();
    let n2 = 2;
    assert_eq!(Solution::get_max_repetitions(s1, n1, s2, n2), 2);

    // 示例 2：
    // 输入：s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
    // 输出：1
    let s1 = "acb".to_string();
    let n1 = 1;
    let s2 = "acb".to_string();
    let n2 = 1;
    assert_eq!(Solution::get_max_repetitions(s1, n1, s2, n2), 1);
}
