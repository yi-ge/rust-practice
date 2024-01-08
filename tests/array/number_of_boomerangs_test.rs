use rust_practice::array::number_of_boomerangs::Solution;

#[test]
fn number_of_boomerangs() {
    // 示例 1：
    // 输入：points = [[0,0],[1,0],[2,0]]
    // 输出：2
    // 解释：两个回旋镖为 [[1,0],[0,0],[2,0]] 和 [[1,0],[2,0],[0,0]]
    let points = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
    assert_eq!(Solution::number_of_boomerangs(points), 2);

    // 示例 2：
    // 输入：points = [[1,1],[2,2],[3,3]]
    // 输出：2
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(Solution::number_of_boomerangs(points), 2);

    // 示例 3：
    // 输入：points = [[1,1]]
    // 输出：0
    let points = vec![vec![1, 1]];
    assert_eq!(Solution::number_of_boomerangs(points), 0);
}
