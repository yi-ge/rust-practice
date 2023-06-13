use rust_practice::array::number_of_unequal_triplets_in_array::Solution;

#[test]
fn unequal_triplets() {
    // 示例 1：
    // 输入：nums = [4,4,2,4,3]
    // 输出：3
    // 解释：下面列出的三元组均满足题目条件：
    // - (0, 2, 4) 因为 4 != 2 != 3
    // - (1, 2, 4) 因为 4 != 2 != 3
    // - (2, 3, 4) 因为 2 != 4 != 3
    // 共计 3 个三元组，返回 3 。
    // 注意 (2, 0, 4) 不是有效的三元组，因为 2 > 0 。
    let nums = vec![4, 4, 2, 4, 3];
    assert_eq!(Solution::unequal_triplets(nums), 3);

    // 示例 2：
    // 输入：nums = [1,1,1,1,1]
    // 输出：0
    // 解释：不存在满足条件的三元组，所以返回 0 。
    let nums = vec![1, 1, 1, 1, 1];
    assert_eq!(Solution::unequal_triplets(nums), 0);
}
