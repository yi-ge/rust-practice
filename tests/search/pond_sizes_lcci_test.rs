use rust_practice::search::pond_sizes_lcci::Solution;

#[test]
fn pond_sizes() {
    // 示例：
    // 输入：
    // [
    //   [0,2,1,0],
    //   [0,1,0,1],
    //   [1,1,0,1],
    //   [0,1,0,1]
    // ]
    // 输出： [1,2,4]
    let land = vec![
        vec![0, 2, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 0, 1],
    ];
    let result = Solution::pond_sizes(land);
    assert_eq!(result, vec![1, 2, 4]);
}
