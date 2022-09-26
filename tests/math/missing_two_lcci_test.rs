use rust_practice::math::missing_two_lcci::Solution;

#[test]
fn missing_two() {
    // 示例 1:
    // 输入: [1]
    // 输出: [2,3]
    assert_eq!(Solution::missing_two(vec![1]), [2, 3]);

    // 示例 2:
    // 输入: [2,3]
    // 输出: [1,4]
    assert_eq!(Solution::missing_two(vec![2, 3]), [4, 1]);
}
