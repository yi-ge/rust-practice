use rust_practice::sort::largest_positive_integer_that_exists_with_its_negative::Solution;

#[test]
fn find_max_k() {
    // 示例 1：
    // 输入：nums = [-1,2,-3,3]
    // 输出：3
    // 解释：3 是数组中唯一一个满足题目要求的 k 。
    let nums = vec![-1, 2, -3, 3];
    assert_eq!(Solution::find_max_k(nums), 3);

    // 示例 2：
    // 输入：nums = [-1,10,6,7,-7,1]
    // 输出：7
    // 解释：数组中存在 1 和 7 对应的负数，7 的值更大。
    let nums = vec![-1, 10, 6, 7, -7, 1];
    assert_eq!(Solution::find_max_k(nums), 7);

    // 示例 3：
    // 输入：nums = [-10,8,6,7,-2,-3]
    // 输出：-1
    // 解释：不存在满足题目要求的 k ，返回 -1 。
    let nums = vec![-10, 8, 6, 7, -2, -3];
    assert_eq!(Solution::find_max_k(nums), -1);
}
