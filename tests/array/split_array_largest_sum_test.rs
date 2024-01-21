use rust_practice::array::split_array_largest_sum::Solution;

#[test]
fn split_array_test() {
    // 示例 1：
    // 输入：nums = [7,2,5,10,8], k = 2
    // 输出：18
    // 解释：
    // 一共有四种方法将 nums 分割为 2 个子数组。
    // 其中最好的方式是将其分为 [7,2,5] 和 [10,8] 。
    // 因为此时这两个子数组各自的和的最大值为18，在所有情况中最小。
    let nums = vec![7, 2, 5, 10, 8];
    let k = 2;
    assert_eq!(Solution::split_array(nums, k), 18);

    // 示例 2：
    // 输入：nums = [1,2,3,4,5], k = 2
    // 输出：9
    let nums = vec![1, 2, 3, 4, 5];
    let k = 2;
    assert_eq!(Solution::split_array(nums, k), 9);

    // 示例 3：
    // 输入：nums = [1,4,4], k = 3
    // 输出：4
    let nums = vec![1, 4, 4];
    let k = 3;
    assert_eq!(Solution::split_array(nums, k), 4);
}
