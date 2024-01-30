use rust_practice::array::minimum_seconds_to_equalize_a_circular_array::Solution;

#[test]
fn minimum_seconds() {
    // 示例 1：
    // 输入：nums = [1,2,1,2]
    // 输出：1
    // 解释：我们可以在 1 秒内将数组变成相等元素：
    // - 第 1 秒，将每个位置的元素分别变为 [nums[3],nums[1],nums[3],nums[3]] 。变化后，nums = [2,2,2,2] 。
    // 1 秒是将数组变成相等元素所需要的最少秒数。
    let nums = vec![1, 2, 1, 2];
    assert_eq!(Solution::minimum_seconds(nums), 1);

    // 示例 2：
    // 输入：nums = [2,1,3,3,2]
    // 输出：2
    // 解释：我们可以在 2 秒内将数组变成相等元素：
    // - 第 1 秒，将每个位置的元素分别变为 [nums[0],nums[2],nums[2],nums[2],nums[3]] 。变化后，nums = [2,3,3,3,3] 。
    // - 第 2 秒，将每个位置的元素分别变为 [nums[1],nums[1],nums[2],nums[3],nums[4]] 。变化后，nums = [3,3,3,3,3] 。
    // 2 秒是将数组变成相等元素所需要的最少秒数。
    let nums = vec![2, 1, 3, 3, 2];
    assert_eq!(Solution::minimum_seconds(nums), 2);

    // 示例 3：
    // 输入：nums = [5,5,5,5]
    // 输出：0
    // 解释：不需要执行任何操作，因为一开始数组中的元素已经全部相等。
    let nums = vec![5, 5, 5, 5];
    assert_eq!(Solution::minimum_seconds(nums), 0);
}
