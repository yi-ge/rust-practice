// 经营摩天轮的最大利润
// https://leetcode.cn/problems/maximum-profit-of-operating-a-centennial-wheel
// INLINE  ../../images/array/maximum_profit_of_operating_a_centennial_wheel.jpeg

pub struct Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let mut profit = 0;
        let mut waiting = 0;
        let mut max_profit = 0;
        let mut rotate = 0;
        let mut max_rotate = 0; // 新增变量来记录达到最大利润时的旋转次数

        for customer in customers {
            waiting += customer;
            let boarding = waiting.min(4);
            waiting -= boarding;
            profit += boarding * boarding_cost - running_cost;
            rotate += 1;
            if profit > max_profit {
                max_profit = profit;
                max_rotate = rotate; // 更新达到最大利润时的旋转次数
            }
        }
        while waiting > 0 {
            let boarding = waiting.min(4);
            waiting -= boarding;
            profit += boarding * boarding_cost - running_cost;
            rotate += 1;
            if profit > max_profit {
                max_profit = profit;
                max_rotate = rotate; // 更新达到最大利润时的旋转次数
            }
        }
        if max_profit <= 0 {
            -1
        } else {
            max_rotate // 返回达到最大利润时的旋转次数
        }
    }
}
