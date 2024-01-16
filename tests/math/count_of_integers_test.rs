use rust_practice::math::count_of_integers::Solution;

#[test]
fn count_test() {
    // 示例 1：
    // 输入：num1 = "1", num2 = "12", min_num = 1, max_num = 8
    // 输出：11
    // 解释：总共有 11 个整数的数位和在 1 到 8 之间，分别是 1,2,3,4,5,6,7,8,10,11 和 12 。所以我们返回 11 。
    let num1 = "1".to_string();
    let num2 = "12".to_string();
    let min_num = 1;
    let max_num = 8;
    assert_eq!(Solution::count(num1, num2, min_num, max_num), 11);

    // 示例 2：
    // 输入：num1 = "1", num2 = "5", min_num = 1, max_num = 5
    // 输出：5
    // 解释：数位和在 1 到 5 之间的 5 个整数分别为 1,2,3,4 和 5 。所以我们返回 5 。
    let num1 = "1".to_string();
    let num2 = "5".to_string();
    let min_num = 1;
    let max_num = 5;
    assert_eq!(Solution::count(num1, num2, min_num, max_num), 5);
}
