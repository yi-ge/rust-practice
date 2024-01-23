use rust_practice::array::longest_alternating_subarray::Solution;

#[test]
fn alternating_subarray_test() {
    // 示例 1：
    // 输入：nums = [2,3,4,3,4]
    // 输出：4
    // 解释：交替子数组有 [3,4] ，[3,4,3] 和 [3,4,3,4] 。最长的子数组为 [3,4,3,4] ，长度为4 。
    let nums = vec![2, 3, 4, 3, 4];
    assert_eq!(Solution::alternating_subarray(nums), 4);

    // 示例 2：
    // 输入：nums = [4,5,6]
    // 输出：2
    // 解释：[4,5] 和 [5,6] 是仅有的两个交替子数组。它们长度都为 2 。
    let nums = vec![4, 5, 6];
    assert_eq!(Solution::alternating_subarray(nums), 2);
}
