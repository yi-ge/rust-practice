use rust_practice::array::merge_intervals::Solution;

#[test]
fn merge() {
    // 示例 1：
    // 输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
    // 输出：[[1,6],[8,10],[15,18]]
    // 解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(Solution::merge(intervals), [[1, 6], [8, 10], [15, 18]]);

    // 示例 2：
    // 输入：intervals = [[1,4],[4,5]]
    // 输出：[[1,5]]
    // 解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
    let intervals = vec![vec![1, 4], vec![4, 5]];
    assert_eq!(Solution::merge(intervals), [[1, 5]]);
}
