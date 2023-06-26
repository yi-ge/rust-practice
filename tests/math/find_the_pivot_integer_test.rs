use rust_practice::math::find_the_pivot_integer::Solution;

#[test]
fn pivot_integer() {
    // 示例 1：
    // 输入：n = 8
    // 输出：6
    // 解释：6 是中枢整数，因为 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21 。
    let n = 8;
    assert_eq!(Solution::pivot_integer(n), 6);

    // 示例 2：
    // 输入：n = 1
    // 输出：1
    // 解释：1 是中枢整数，因为 1 = 1 。
    let n = 1;
    assert_eq!(Solution::pivot_integer(n), 1);

    // 示例 3：
    // 输入：n = 4
    // 输出：-1
    // 解释：可以证明不存在满足题目要求的整数。
    let n = 4;
    assert_eq!(Solution::pivot_integer(n), -1);
}
