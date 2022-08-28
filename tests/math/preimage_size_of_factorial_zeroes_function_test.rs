use rust_practice::math::preimage_size_of_factorial_zeroes_function::Solution;

#[test]
fn preimage_size_fzf() {
    // 示例 1：
    // 输入：k = 0
    // 输出：5
    // 解释：0!, 1!, 2!, 3!, 和 4! 均符合 k = 0 的条件。
    assert_eq!(Solution::preimage_size_fzf(0), 5);

    // 示例 2：
    // 输入：k = 5
    // 输出：0
    // 解释：没有匹配到这样的 x!，符合 k = 5 的条件。
    assert_eq!(Solution::preimage_size_fzf(5), 0);

    // 示例 3:
    // 输入: k = 3
    // 输出: 5
    assert_eq!(Solution::preimage_size_fzf(3), 5);
}
