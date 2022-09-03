// 商品折扣后的最终价格
// https://leetcode.cn/problems/final-prices-with-a-special-discount-in-a-shop
// INLINE  ../../images/heap/final_prices_with_a_special_discount_in_a_shop.jpeg

pub struct Solution;

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        for i in 0..prices.len() {
            for j in i + 1..prices.len() {
                if prices[j] <= prices[i] {
                    prices[i] = prices[i] - prices[j];
                    break;
                }
            }
        }

        prices
    }
}
