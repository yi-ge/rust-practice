use rust_practice::math::smallest_even_multiple::Solution;

#[test]
fn smallest_even_multiple() {
    // 示例 1：
    // 输入：n = 5
    // 输出：10
    // 解释：5 和 2 的最小公倍数是 10 。
    assert_eq!(Solution::smallest_even_multiple(5), 10);

    // 示例 2：
    // 输入：n = 6
    // 输出：6
    // 解释：6 和 2 的最小公倍数是 6 。注意数字会是它自身的倍数。
    assert_eq!(Solution::smallest_even_multiple(6), 6);
}
