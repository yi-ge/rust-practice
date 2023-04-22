use rust_practice::array::number_of_arithmetic_triplets::Solution;

#[test]
fn arithmetic_triplets() {
    // 示例 1：
    // 输入：nums = [0,1,4,6,7,10], diff = 3
    // 输出：2
    // 解释：
    // (1, 2, 4) 是算术三元组：7 - 4 == 3 且 4 - 1 == 3 。
    // (2, 4, 5) 是算术三元组：10 - 7 == 3 且 7 - 4 == 3 。
    let nums1 = vec![0, 1, 4, 6, 7, 10];
    let diff1 = 3;
    assert_eq!(Solution::arithmetic_triplets(nums1, diff1), 2);

    // 示例 2：
    // 输入：nums = [4,5,6,7,8,9], diff = 2
    // 输出：2
    // 解释：
    // (0, 2, 4) 是算术三元组：8 - 6 == 2 且 6 - 4 == 2 。
    // (1, 3, 5) 是算术三元组：9 - 7 == 2 且 7 - 5 == 2 。
    let nums2 = vec![4, 5, 6, 7, 8, 9];
    let diff2 = 2;
    assert_eq!(Solution::arithmetic_triplets(nums2, diff2), 2);
}
