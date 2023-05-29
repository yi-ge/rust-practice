use rust_practice::array::average_value_of_even_numbers_that_are_divisible_by_three::Solution;

#[test]
fn average_value() {
    // 示例 1：
    // 输入：nums = [1,3,6,10,12,15]
    // 输出：9
    // 解释：6 和 12 是可以被 3 整除的偶数。(6 + 12) / 2 = 9 。
    let nums = vec![1, 3, 6, 10, 12, 15];
    assert_eq!(Solution::average_value(nums), 9);

    // 示例 2：
    // 输入：nums = [1,2,4,7,10]
    // 输出：0
    // 解释：不存在满足题目要求的整数，所以返回 0 。
    let nums = vec![1, 2, 4, 7, 10];
    assert_eq!(Solution::average_value(nums), 0);
}
