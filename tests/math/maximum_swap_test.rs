use rust_practice::math::maximum_swap::Solution;

#[test]
fn maximum_swap_test() {
    // 示例 1 :
    // 输入: 2736
    // 输出: 7236
    // 解释: 交换数字2和数字7。
    let num = 2736;
    let result = Solution::maximum_swap(num);
    assert_eq!(result, 7236);

    // 示例 2 :
    // 输入: 9973
    // 输出: 9973
    // 解释: 不需要交换。
    let num = 9973;
    let result = Solution::maximum_swap(num);
    assert_eq!(result, 9973);
}
