// 可被三整除的偶数的平均值
// https://leetcode.cn/problems/average-value-of-even-numbers-that-are-divisible-by-three
// INLINE  ../../images/array/average_value_of_even_numbers_that_are_divisible_by_three.jpeg

pub struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        for num in nums {
            if num % 2 == 0 && num % 3 == 0 {
                sum += num;
                count += 1;
            }
        }
        if count == 0 {
            return 0;
        }
        sum / count
    }
}
