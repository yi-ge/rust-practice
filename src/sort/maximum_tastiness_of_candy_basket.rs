// 礼盒的最大甜蜜度
// https://leetcode.cn/problems/maximum-tastiness-of-candy-basket
// INLINE  ../../images/sort/maximum_tastiness_of_candy_basket.jpeg

pub struct Solution;

impl Solution {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        // 对价格进行排序
        let mut price = price;
        price.sort();
        // 获取商品数量
        let n = price.len();
        // 定义二分搜索范围的左右边界
        let mut left = 0;
        let mut right = price[n - 1] - price[0];
        // 在[left, right]范围内进行二分搜索
        while left < right {
            // 获取二分搜索范围的中间值
            let mid = (left + right + 1) >> 1;
            // 检查是否可以以mid的美味度购买至少k件商品
            if Self::check(&price, k, mid) {
                // 如果可以，更新二分搜索范围的左边界
                left = mid;
            } else {
                // 否则，更新二分搜索范围的右边界
                right = mid - 1;
            }
        }
        // 返回最大美味度
        return left;
    }

    fn check(price: &Vec<i32>, k: i32, tastiness: i32) -> bool {
        // 初始化前一件商品的价格为最小整数
        let mut prev = i32::MIN >> 1;
        // 初始化购买的商品数量为0
        let mut cnt = 0;
        // 遍历商品
        for &p in price.iter() {
            // 如果当前商品的价格减去前一件商品的价格大于等于美味度
            if p - prev >= tastiness {
                // 增加购买的商品数量
                cnt += 1;
                // 更新前一件商品的价格
                prev = p;
            }
        }
        // 检查是否可以购买至少k件商品
        return cnt >= k;
    }
}
