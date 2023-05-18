use rust_practice::array::adding_two_negabinary_numbers::Solution;

#[test]
fn add_negabinary() {
    // 示例 1：
    // 输入：arr1 = [1,1,1,1,1], arr2 = [1,0,1]
    // 输出：[1,0,0,0,0]
    // 解释：arr1 表示 11，arr2 表示 5，输出表示 16 。
    let arr1 = vec![1, 1, 1, 1, 1];
    let arr2 = vec![1, 0, 1];
    let result = Solution::add_negabinary(arr1, arr2);
    assert_eq!(result, vec![1, 0, 0, 0, 0]);

    // 示例 2：
    // 输入：arr1 = [0], arr2 = [0]
    // 输出：[0]
    let arr1 = vec![0];
    let arr2 = vec![0];
    let result = Solution::add_negabinary(arr1, arr2);
    assert_eq!(result, vec![0]);

    // 示例 3：
    // 输入：arr1 = [0], arr2 = [1]
    // 输出：[1]
    let arr1 = vec![0];
    let arr2 = vec![1];
    let result = Solution::add_negabinary(arr1, arr2);
    assert_eq!(result, vec![1]);
}
