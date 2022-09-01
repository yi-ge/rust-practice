use rust_practice::graphs::find_the_town_judge::Solution;

#[test]
fn find_judge() {
    // 示例 1：
    // 输入：n = 2, trust = [[1,2]]
    // 输出：2
    let trust = vec![vec![1, 2]];
    assert_eq!(Solution::find_judge(2, trust), 2);

    // 示例 2：
    // 输入：n = 3, trust = [[1,3],[2,3]]
    // 输出：3
    let trust = vec![vec![1, 3], vec![2, 3]];
    assert_eq!(Solution::find_judge(3, trust), 3);

    // 示例 3：
    // 输入：n = 3, trust = [[1,3],[2,3],[3,1]]
    // 输出：-1
    let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
    assert_eq!(Solution::find_judge(3, trust), -1);
}
