use rust_practice::array::two_sum::Solution;

#[test]
fn two_sum() {
    // 示例 1：
    // 输入：nums = [2,7,11,15], target = 9
    // 输出：[0,1]
    // 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let res = Solution::two_sum(nums, target);
    assert_eq!(res, vec![1, 0][..]);

    // 示例 2：
    // 输入：nums = [3,2,4], target = 6
    // 输出：[1,2]
    let nums = vec![3, 2, 4];
    let target = 6;
    let res = Solution::two_sum(nums, target);
    assert_eq!(res, vec![2, 1][..]);

    // 示例 3：
    // 输入：nums = [3,3], target = 6
    // 输出：[0,1]
    let nums = vec![3, 3];
    let target = 6;
    let res = Solution::two_sum(nums, target);
    assert_eq!(res, vec![1, 0][..]);
}
