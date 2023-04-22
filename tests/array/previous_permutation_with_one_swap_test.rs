use rust_practice::array::previous_permutation_with_one_swap::Solution;

#[test]
fn prev_perm_opt1() {
    // 示例 1：
    // 输入：arr = [3,2,1]
    // 输出：[3,1,2]
    // 解释：交换 2 和 1
    let arr1 = vec![3, 2, 1];
    let exp1 = vec![3, 1, 2];
    assert_eq!(Solution::prev_perm_opt1(arr1), exp1);

    // 示例 2：
    // 输入：arr = [1,1,5]
    // 输出：[1,1,5]
    // 解释：已经是最小排列
    let arr2 = vec![1, 1, 5];
    let exp2 = vec![1, 1, 5];
    assert_eq!(Solution::prev_perm_opt1(arr2), exp2);

    // 示例 3：
    // 输入：arr = [1,9,4,6,7]
    // 输出：[1,7,4,6,9]
    // 解释：交换 9 和 7
    let arr3 = vec![1, 9, 4, 6, 7];
    let exp3 = vec![1, 7, 4, 6, 9];
    assert_eq!(Solution::prev_perm_opt1(arr3), exp3);

    // 边界情况1：空数组
    let arr4 = vec![];
    let exp4 = vec![];
    assert_eq!(Solution::prev_perm_opt1(arr4), exp4);

    // 边界情况2：只有一个元素的数组
    let arr5 = vec![1];
    let exp5 = vec![1];
    assert_eq!(Solution::prev_perm_opt1(arr5), exp5);
}
