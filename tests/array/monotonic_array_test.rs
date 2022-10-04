use rust_practice::array::monotonic_array::Solution;

#[test]
fn is_monotonic() {
    // 示例 1：
    // 输入：nums = [1,2,2,3]
    // 输出：true
    let nums = vec![1, 2, 2, 3];
    assert!(Solution::is_monotonic(nums));

    // 示例 2：
    // 输入：nums = [6,5,4,4]
    // 输出：true
    let nums = vec![6, 5, 4, 4];
    assert!(Solution::is_monotonic(nums));

    // 示例 3：
    // 输入：nums = [1,3,2]
    // 输出：false
    let nums = vec![1, 3, 2];
    assert!(!Solution::is_monotonic(nums));
}
