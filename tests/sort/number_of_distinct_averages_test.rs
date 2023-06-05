use rust_practice::sort::number_of_distinct_averages::Solution;

#[test]
fn distinct_averages() {
    // 示例 1：
    // 输入：nums = [4,1,4,0,3,5]
    // 输出：2
    // 解释：
    // 1. 删除 0 和 5 ，平均值是 (0 + 5) / 2 = 2.5 ，现在 nums = [4,1,4,3] 。
    // 2. 删除 1 和 4 ，平均值是 (1 + 4) / 2 = 2.5 ，现在 nums = [4,3] 。
    // 3. 删除 3 和 4 ，平均值是 (3 + 4) / 2 = 3.5 。
    // 2.5 ，2.5 和 3.5 之中总共有 2 个不同的数，我们返回 2 。
    let nums = vec![4, 1, 4, 0, 3, 5];
    let result = Solution::distinct_averages(nums);
    assert_eq!(result, 2);

    // 示例 2：
    // 输入：nums = [1,100]
    // 输出：1
    // 解释：
    // 删除 1 和 100 后只有一个平均值，所以我们返回 1 。
    let nums = vec![1, 100];
    let result = Solution::distinct_averages(nums);
    assert_eq!(result, 1);
}
