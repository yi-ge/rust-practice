use rust_practice::array::maximum_size_subarray_sum_equals_k::Solution;

#[test]
fn max_sub_array_len() {
    // 示例 1:
    // 输入: nums = [1,-1,5,-2,3], k = 3
    // 输出: 4
    // 解释: 子数组 [1, -1, 5, -2] 和等于 3，且长度最长。
    let nums = vec![1, -1, 5, -2, 3];
    let k = 3;
    assert_eq!(Solution::max_sub_array_len(nums, k), 4);

    // 示例 2:
    // 输入: nums = [-2,-1,2,1], k = 1
    // 输出: 2
    // 解释: 子数组 [-1, 2] 和等于 1，且长度最长。
    let nums = vec![-2, -1, 2, 1];
    let k = 1;
    assert_eq!(Solution::max_sub_array_len(nums, k), 2);

    let nums = vec![1, 1, 0];
    let k = 1;
    assert_eq!(Solution::max_sub_array_len(nums, k), 2);
}
