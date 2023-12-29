use rust_practice::sort::buy_two_chocolates::Solution;

#[test]
fn buy_choco() {
    // 示例 1：
    // 输入：prices = [1,2,2], money = 3
    // 输出：0
    // 解释：分别购买价格为 1 和 2 的巧克力。你剩下 3 - 3 = 0 块钱。所以我们返回 0 。
    let prices = vec![1, 2, 2];
    let money = 3;
    assert_eq!(Solution::buy_choco(prices, money), 0);

    // 示例 2：
    // 输入：prices = [3,2,3], money = 3
    // 输出：3
    // 解释：购买任意 2 块巧克力都会超过你拥有的钱数，所以我们返回 3 。
    let prices = vec![3, 2, 3];
    let money = 3;
    assert_eq!(Solution::buy_choco(prices, money), 3);
}
