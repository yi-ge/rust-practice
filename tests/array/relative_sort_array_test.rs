use rust_practice::array::relative_sort_array::Solution;

#[test]
fn relative_sort_array() {
    // 示例 1：
    // 输入：arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
    // 输出：[2,2,2,1,4,3,3,9,6,7,19]
    assert_eq!(
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );

    // 示例  2:
    // 输入：arr1 = [28,6,22,8,44,17], arr2 = [22,28,8,6]
    // 输出：[22,28,8,6,17,44]
    assert_eq!(
        Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
        vec![22, 28, 8, 6, 17, 44]
    );
}
