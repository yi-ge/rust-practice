use rust_practice::math::convert_to_base_2::Solution;

#[test]
fn base_neg2() {
    // 示例 1：
    // 输入：n = 2
    // 输出："110"
    // 解释：(-2)2 + (-2)1 = 2
    assert_eq!(Solution::base_neg2(2), "110");

    // 示例 2：
    // 输入：n = 3
    // 输出："111"
    // 解释：(-2)2 + (-2)1 + (-2)0 = 3
    assert_eq!(Solution::base_neg2(3), "111");

    // 示例 3：
    // 输入：n = 4
    // 输出："100"
    // 解释：(-2)2 = 4
    assert_eq!(Solution::base_neg2(4), "100");

    // Test case 1: n = 0
    assert_eq!(Solution::base_neg2(0), "0");

    // Test case 2: n = 1
    assert_eq!(Solution::base_neg2(1), "1");
}
