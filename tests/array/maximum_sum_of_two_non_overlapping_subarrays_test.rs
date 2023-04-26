use rust_practice::array::maximum_sum_of_two_non_overlapping_subarrays::Solution;

#[test]
fn max_sum_two_no_overlap() {
    // 示例 1：
    // 输入：nums = [0,6,5,2,2,5,1,9,4], firstLen = 1, secondLen = 2
    // 输出：20
    // 解释：子数组的一种选择中，[9] 长度为 1，[6,5] 长度为 2。
    let nums = vec![0, 6, 5, 2, 2, 5, 1, 9, 4];
    let first_len = 1;
    let second_len = 2;
    assert_eq!(
        Solution::max_sum_two_no_overlap(nums, first_len, second_len),
        20
    );

    // 示例 2：
    // 输入：nums = [3,8,1,3,2,1,8,9,0], firstLen = 3, secondLen = 2
    // 输出：29
    // 解释：子数组的一种选择中，[3,8,1] 长度为 3，[8,9] 长度为 2。
    let nums = vec![3, 8, 1, 3, 2, 1, 8, 9, 0];
    let first_len = 3;
    let second_len = 2;
    assert_eq!(
        Solution::max_sum_two_no_overlap(nums, first_len, second_len),
        29
    );

    // 示例 3：
    // 输入：nums = [2,1,5,6,0,9,5,0,3,8], firstLen = 4, secondLen = 3
    // 输出：31
    // 解释：子数组的一种选择中，[5,6,0,9] 长度为 4，[0,3,8] 长度为 3。
    let nums = vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8];
    let first_len = 4;
    let second_len = 3;
    assert_eq!(
        Solution::max_sum_two_no_overlap(nums, first_len, second_len),
        31
    );
}
