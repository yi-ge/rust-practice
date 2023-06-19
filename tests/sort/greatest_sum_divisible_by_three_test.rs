use rust_practice::sort::greatest_sum_divisible_by_three::Solution;

#[test]
fn max_sum_div_three() {
    // 示例 1：
    // 输入：nums = [3,6,5,1,8]
    // 输出：18
    // 解释：选出数字 3, 6, 1 和 8，它们的和是 18（可被 3 整除的最大和）。
    let nums = vec![3, 6, 5, 1, 8];
    assert_eq!(Solution::max_sum_div_three(nums), 18);

    // 示例 2：
    // 输入：nums = [4]
    // 输出：0
    // 解释：4 不能被 3 整除，所以无法选出数字，返回 0。
    let nums = vec![4];
    assert_eq!(Solution::max_sum_div_three(nums), 0);

    // 示例 3：
    // 输入：nums = [1,2,3,4,4]
    // 输出：12
    // 解释：选出数字 1, 3, 4 以及 4，它们的和是 12（可被 3 整除的最大和）。
    let nums = vec![1, 2, 3, 4, 4];
    assert_eq!(Solution::max_sum_div_three(nums), 12);
}
