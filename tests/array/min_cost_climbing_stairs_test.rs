use rust_practice::array::min_cost_climbing_stairs::Solution;

#[test]
fn min_cost_climbing_stairs() {
    // 示例 1：
    // 输入：cost = [10,15,20]
    // 输出：15
    // 解释：你将从下标为 1 的台阶开始。
    // - 支付 15 ，向上爬两个台阶，到达楼梯顶部。
    // 总花费为 15 。
    let cost = vec![10, 15, 20];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);

    // 示例 2：
    // 输入：cost = [1,100,1,1,1,100,1,1,100,1]
    // 输出：6
    // 解释：你将从下标为 0 的台阶开始。
    // - 支付 1 ，向上爬两个台阶，到达下标为 2 的台阶。
    // - 支付 1 ，向上爬两个台阶，到达下标为 4 的台阶。
    // - 支付 1 ，向上爬两个台阶，到达下标为 6 的台阶。
    // - 支付 1 ，向上爬一个台阶，到达下标为 7 的台阶。
    // - 支付 1 ，向上爬两个台阶，到达下标为 9 的台阶。
    // - 支付 1 ，向上爬一个台阶，到达楼梯顶部。
    // 总花费为 6 。
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
}
