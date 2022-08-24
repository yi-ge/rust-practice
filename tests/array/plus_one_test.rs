use rust_practice::array::plus_one::Solution;

#[test]
fn plus_one() {
    // 示例 1：
    // 输入：digits = [1,2,3]
    // 输出：[1,2,4]
    // 解释：输入数组表示数字 123。
    assert_eq!(&Solution::plus_one(vec![1, 2, 3])[..], &[1, 2, 4][..]);

    // 示例 2：
    // 输入：digits = [4,3,2,1]
    // 输出：[4,3,2,2]
    // 解释：输入数组表示数字 4321。
    assert_eq!(&Solution::plus_one(vec![4, 3, 2, 1])[..], &[4, 3, 2, 2][..]);

    // 示例 3：
    // 输入：digits = [0]
    // 输出：[1]
    assert_eq!(&Solution::plus_one(vec![0])[..], &[1][..]);
}
