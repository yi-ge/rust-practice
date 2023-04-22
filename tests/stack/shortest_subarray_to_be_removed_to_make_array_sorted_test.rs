use rust_practice::stack::shortest_subarray_to_be_removed_to_make_array_sorted::Solution;

#[test]
fn find_length_of_shortest_subarray() {
    // 示例 1：
    // 输入：arr = [1,2,3,10,4,2,3,5]
    // 输出：3
    // 解释：我们需要删除的最短子数组是 [10,4,2] ，长度为 3 。剩余元素形成非递减数组 [1,2,3,3,5] 。
    // 另一个正确的解为删除子数组 [3,10,4] 。
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
        3
    );

    // 示例 2：
    // 输入：arr = [5,4,3,2,1]
    // 输出：4
    // 解释：由于数组是严格递减的，我们只能保留一个元素。所以我们需要删除长度为 4 的子数组，要么删除 [5,4,3,2]，要么删除 [4,3,2,1]。
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
        4
    );

    // 示例 3：
    // 输入：arr = [1,2,3]
    // 输出：0
    // 解释：数组已经是非递减的了，我们不需要删除任何元素。
    assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0);

    // 示例 4：
    // 输入：arr = [1]
    // 输出：0
    assert_eq!(Solution::find_length_of_shortest_subarray(vec![1]), 0);
}
