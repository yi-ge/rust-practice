use rust_practice::math::number_of_common_factors::Solution;

#[test]
fn common_factors() {
    // 示例 1：
    // 输入：a = 12, b = 6
    // 输出：4
    // 解释：12 和 6 的公因子是 1、2、3、6 。
    assert_eq!(Solution::common_factors(12, 6), 4);

    // 示例 2：
    // 输入：a = 25, b = 30
    // 输出：2
    // 解释：25 和 30 的公因子是 1、5 。
    assert_eq!(Solution::common_factors(25, 30), 2);
}
