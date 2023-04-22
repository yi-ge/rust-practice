use rust_practice::array::most_frequent_even_element::Solution;

#[test]
fn most_frequent_even() {
    // 示例 1：
    // 输入：nums = [0,1,2,2,4,4,1]
    // 输出：2
    // 解释：
    // 数组中的偶数元素为 0、2 和 4 ，在这些元素中，2 和 4 出现次数最多。
    // 返回最小的那个，即返回 2 。
    let nums = vec![0, 1, 2, 2, 4, 4, 1];
    assert_eq!(Solution::most_frequent_even(nums), 2);

    // 示例 2：
    // 输入：nums = [4,4,4,9,2,4]
    // 输出：4
    // 解释：4 是出现最频繁的偶数元素。
    let nums = vec![4, 4, 4, 9, 2, 4];
    assert_eq!(Solution::most_frequent_even(nums), 4);

    // 示例 3：
    // 输入：nums = [29,47,21,41,13,37,25,7]
    // 输出：-1
    // 解释：不存在偶数元素。
    let nums = vec![29, 47, 21, 41, 13, 37, 25, 7];
    assert_eq!(Solution::most_frequent_even(nums), -1);
}
