use rust_practice::array::longest_arithmetic_subsequence::Solution;

#[test]
fn longest_arith_seq_length() {
    // 示例 1：
    // 输入：nums = [3,6,9,12]
    // 输出：4
    // 解释：
    // 整个数组是公差为 3 的等差数列。
    let nums = vec![3, 6, 9, 12];
    assert_eq!(Solution::longest_arith_seq_length(nums), 4);

    // 示例 2：
    // 输入：nums = [9,4,7,2,10]
    // 输出：3
    // 解释：
    // 最长的等差子序列是 [4,7,10]。
    let nums = vec![9, 4, 7, 2, 10];
    assert_eq!(Solution::longest_arith_seq_length(nums), 3);

    // 示例 3：
    // 输入：nums = [20,1,15,3,10,5,8]
    // 输出：4
    // 解释：
    // 最长的等差子序列是 [20,15,10,5]。
    let nums = vec![20, 1, 15, 3, 10, 5, 8];
    assert_eq!(Solution::longest_arith_seq_length(nums), 4);
}
