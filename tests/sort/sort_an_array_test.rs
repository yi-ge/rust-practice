use rust_practice::sort::sort_an_array::Solution;

#[test]
fn sort_array() {
    // 示例 1：
    // 输入：nums = [5,2,3,1]
    // 输出：[1,2,3,5]
    let nums = vec![5, 2, 3, 1];
    assert_eq!(Solution::sort_array(nums), [1, 2, 3, 5]);

    // 示例 2：
    // 输入：nums = [5,1,1,2,0,0]
    // 输出：[0,0,1,1,2,5]
    let nums = vec![5, 1, 1, 2, 0, 0];
    assert_eq!(Solution::sort_array(nums), [0, 0, 1, 1, 2, 5]);

    let nums = vec![];
    assert_eq!(Solution::sort_array(nums), []);

    let nums = vec![-2, 3, -5];
    assert_eq!(Solution::sort_array(nums), [-5, -2, 3]);
}
