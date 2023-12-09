use rust_practice::math::next_greater_numerically_balanced_number::Solution;

#[test]
fn next_beautiful_number() {
    // 示例 1：
    // 输入：n = 1
    // 输出：22
    // 解释：
    // 22 是一个数值平衡数，因为：
    // - 数字 2 出现 2 次
    // 这也是严格大于 1 的最小数值平衡数。
    let n = 1;
    assert_eq!(Solution::next_beautiful_number(n), 22);

    // 示例 2：
    // 输入：n = 1000
    // 输出：1333
    // 解释：
    // 1333 是一个数值平衡数，因为：
    // - 数字 1 出现 1 次。
    // - 数字 3 出现 3 次。
    // 这也是严格大于 1000 的最小数值平衡数。
    // 注意，1022 不能作为本输入的答案，因为数字 0 的出现次数超过了 0 。
    let n = 1000;
    assert_eq!(Solution::next_beautiful_number(n), 1333);

    // 示例 3：
    // 输入：n = 3000
    // 输出：3133
    // 解释：
    // 3133 是一个数值平衡数，因为：
    // - 数字 1 出现 1 次。
    // - 数字 3 出现 3 次。
    // 这也是严格大于 3000 的最小数值平衡数。
    let n = 3000;
    assert_eq!(Solution::next_beautiful_number(n), 3133);
}
