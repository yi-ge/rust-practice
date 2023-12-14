use rust_practice::array::stamping_the_grid::Solution;

#[test]
fn possible_to_stamp() {
    // 示例 1：
    // 输入：grid = [[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0]], stampHeight = 4, stampWidth = 3
    // 输出：true
    // 解释：我们放入两个有重叠部分的邮票（图中标号为 1 和 2），它们能覆盖所有与空格子。
    let grid = vec![
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0],
    ];
    let stamp_height = 4;
    let stamp_width = 3;
    assert_eq!(
        Solution::possible_to_stamp(grid, stamp_height, stamp_width),
        true
    );

    // 示例 2：
    // 输入：grid = [[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]], stampHeight = 2, stampWidth = 2
    // 输出：false
    // 解释：没办法放入邮票覆盖所有的空格子，且邮票不超出网格图以外。
    let grid = vec![
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 1],
    ];
    let stamp_height = 2;
    let stamp_width = 2;
    assert_eq!(
        Solution::possible_to_stamp(grid, stamp_height, stamp_width),
        false
    );
}
