use rust_practice::array::maximum_length_of_pair_chain::Solution;

#[test]
fn find_longest_chain() {
    // 示例：
    // 输入：[[1,2], [2,3], [3,4]]
    // 输出：2
    // 解释：最长的数对链是 [1,2] -> [3,4]
    let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    assert_eq!(Solution::find_longest_chain(pairs), 2);
}
