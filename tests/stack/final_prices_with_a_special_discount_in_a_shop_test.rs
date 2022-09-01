use rust_practice::stack::final_prices_with_a_special_discount_in_a_shop::Solution;

#[test]
fn final_prices() {
    // 示例 1：
    // 输入：prices = [8,4,6,2,3]
    // 输出：[4,2,4,2,3]
    // 解释：
    // 商品 0 的价格为 price[0]=8 ，你将得到 prices[1]=4 的折扣，所以最终价格为 8 - 4 = 4 。
    // 商品 1 的价格为 price[1]=4 ，你将得到 prices[3]=2 的折扣，所以最终价格为 4 - 2 = 2 。
    // 商品 2 的价格为 price[2]=6 ，你将得到 prices[3]=2 的折扣，所以最终价格为 6 - 2 = 4 。
    // 商品 3 和 4 都没有折扣。
    let prices = vec![8, 4, 6, 2, 3];
    assert_eq!(Solution::final_prices(prices), &vec![4, 2, 4, 2, 3][..]);

    // 示例 2：
    // 输入：prices = [1,2,3,4,5]
    // 输出：[1,2,3,4,5]
    // 解释：在这个例子中，所有商品都没有折扣。
    let prices = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::final_prices(prices), &vec![1, 2, 3, 4, 5][..]);

    // 示例 3：
    // 输入：prices = [10,1,1,6]
    // 输出：[9,0,1,6]
    let prices = vec![10, 1, 1, 6];
    assert_eq!(Solution::final_prices(prices), &vec![9, 0, 1, 6][..]);
}
