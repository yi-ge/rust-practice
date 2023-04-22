use rust_practice::array::partition_array_for_maximum_sum::Solution;

#[test]
fn max_sum_after_partitioning() {
    // 示例 1：
    // 输入：arr = [1,15,7,9,2,5,10], k = 3
    // 输出：84
    // 解释：数组变为 [15,15,15,9,10,10,10]
    let arr = vec![1, 15, 7, 9, 2, 5, 10];
    let k = 3;
    assert_eq!(Solution::max_sum_after_partitioning(arr, k), 84);

    // 示例 2：
    // 输入：arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
    // 输出：83
    let arr = vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3];
    let k = 4;
    assert_eq!(Solution::max_sum_after_partitioning(arr, k), 83);

    // 示例 3：
    // 输入：arr = [1], k = 1
    // 输出：1
    let arr = vec![1];
    let k = 1;
    assert_eq!(Solution::max_sum_after_partitioning(arr, k), 1);
}
