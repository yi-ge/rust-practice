// 可被三整除的偶数的平均值
// https://leetcode.cn/problems/average-value-of-even-numbers-that-are-divisible-by-three
// INLINE  ../../images/array/average_value_of_even_numbers_that_are_divisible_by_three.jpeg

pub struct Solution;

impl Solution {
    // 定义一个函数，参数为一个 i32 类型的数组，返回值为 i32 类型
    pub fn average_value(nums: Vec<i32>) -> i32 {
        // 定义一个 i32 类型的变量 sum，初始值为 0
        let mut sum = 0;
        // 定义一个 i32 类型的变量 count，初始值为 0
        let mut count = 0;
        // 遍历数组 nums，将每个元素依次赋值给变量 num
        for num in nums {
            // 判断当前 num 是否是偶数且能被 3 整除
            if num % 2 == 0 && num % 3 == 0 {
                // 如果是，则将 num 加入到 sum 中
                sum += num;
                // count 加 1
                count += 1;
            }
        }
        // 如果 count 为 0，则返回 0
        if count == 0 {
            return 0;
        }
        // 返回 sum 除以 count 的结果
        sum / count
    }
}
