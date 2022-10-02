use rust_practice::array::kth_largest_element_in_an_array::Solution;

#[test]
fn find_kth_largest() {
    // 示例 1:
    // 输入: [3,2,1,5,6,4], k = 2
    // 输出: 5
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    assert_eq!(Solution::find_kth_largest(nums, k), 5);

    // 示例 2:
    // 输入: [3,2,3,1,2,4,5,5,6], k = 4
    // 输出: 4
    let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let k = 4;
    assert_eq!(Solution::find_kth_largest(nums, k), 4);

    let nums = vec![1];
    let k = 1;
    assert_eq!(Solution::find_kth_largest(nums, k), 1);
}
