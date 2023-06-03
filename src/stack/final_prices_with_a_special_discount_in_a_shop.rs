// 商品折扣后的最终价格
// https://leetcode.cn/problems/final-prices-with-a-special-discount-in-a-shop
// INLINE  ../../images/heap/final_prices_with_a_special_discount_in_a_shop.jpeg

pub struct Solution;

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        // 遍历价格数组
        for i in 0..prices.len() {
            // 从当前位置的下一个位置开始遍历
            for j in i + 1..prices.len() {
                // 如果后面的价格小于等于当前价格
                if prices[j] <= prices[i] {
                    // 折扣后的价格等于当前价格减去后面第一个小于等于当前价格的价格
                    prices[i] = prices[i] - prices[j];
                    break;
                }
            }
        }

        // 返回折扣后的价格数组
        prices
    }
}
