use rust_practice::array::non_overlapping_intervals::Solution;

#[test]
fn erase_overlap_intervals() {
    // 示例 1:
    // 输入: intervals = [[1,2],[2,3],[3,4],[1,3]]
    // 输出: 1
    // 解释: 移除 [1,3] 后，剩下的区间没有重叠。
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );

    // 示例 2:
    // 输入: intervals = [ [1,2], [1,2], [1,2] ]
    // 输出: 2
    // 解释: 你需要移除两个 [1,2] 来使剩下的区间没有重叠。
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
        2
    );

    // 示例 3:
    // 输入: intervals = [ [1,2], [2,3] ]
    // 输出: 0
    // 解释: 你不需要移除任何区间，因为它们已经是无重叠的了。
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
        0
    );
}
