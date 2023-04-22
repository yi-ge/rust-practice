use rust_practice::array::find_subarrays_with_equal_sum::Solution;

#[test]
fn find_subarrays() {
    // 示例 1：
    // 输入：nums = [4,2,4]
    // 输出：true
    // 解释：元素为 [4,2] 和 [2,4] 的子数组有相同的和 6 。
    let nums1 = vec![4, 2, 4];
    assert_eq!(Solution::find_subarrays(nums1), true);

    // 示例 2：
    // 输入：nums = [1,2,3,4,5]
    // 输出：false
    // 解释：没有长度为 2 的两个子数组和相等。
    let nums2 = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::find_subarrays(nums2), false);

    // 示例 3：
    // 输入：nums = [0,0,0]
    // 输出：true
    // 解释：子数组 [nums[0],nums[1]] 和 [nums[1],nums[2]] 的和相等，都为 0 。
    // 注意即使子数组的元素相同，这两个子数组也视为不相同的子数组，因为它们在原数组中的起始位置不同。
    let nums3 = vec![0, 0, 0];
    assert_eq!(Solution::find_subarrays(nums3), true);
}
