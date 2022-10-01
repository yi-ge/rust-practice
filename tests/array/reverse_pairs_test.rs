use rust_practice::array::reverse_pairs::Solution;

#[test]
fn reverse_pairs() {
    // 示例 1:
    // 输入: [1,3,2,3,1]
    // 输出: 2
    assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);

    // 示例 2:
    // 输入: [2,4,3,5,1]
    // 输出: 3
    assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
}
