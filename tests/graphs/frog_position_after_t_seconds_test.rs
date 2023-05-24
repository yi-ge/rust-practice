use approx::assert_abs_diff_eq;
use rust_practice::graphs::frog_position_after_t_seconds::Solution;

#[test]
fn frog_position() {
    // 示例 1：
    // 输入：n = 7, edges = [[1,2],[1,3],[1,7],[2,4],[2,6],[3,5]], t = 2, target = 4
    // 输出：0.16666666666666666
    // 解释：上图显示了青蛙的跳跃路径。青蛙从顶点 1 起跳，第 1 秒 有 1/3 的概率跳到顶点 2 ，然后第 2 秒 有 1/2 的概率跳到顶点 4，因此青蛙在 2 秒后位于顶点 4 的概率是 1/3 * 1/2 = 1/6 = 0.16666666666666666 。
    let n = 7;
    let edges = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 7],
        vec![2, 4],
        vec![2, 6],
        vec![3, 5],
    ];
    let t = 2;
    let target = 4;
    let result = Solution::frog_position(n, edges.clone(), t, target);
    assert_abs_diff_eq!(result, 0.16666666666666666, epsilon = 1e-5);

    // 示例 2：
    // 输入：n = 7, edges = [[1,2],[1,3],[1,7],[2,4],[2,6],[3,5]], t = 1, target = 7
    // 输出：0.3333333333333333
    // 解释：上图显示了青蛙的跳跃路径。青蛙从顶点 1 起跳，有 1/3 = 0.3333333333333333 的概率能够 1 秒 后跳到顶点 7 。
    let n = 7;
    let edges = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 7],
        vec![2, 4],
        vec![2, 6],
        vec![3, 5],
    ];
    let t = 1;
    let target = 7;
    let result = Solution::frog_position(n, edges, t, target);
    assert_abs_diff_eq!(result, 0.3333333333333333, epsilon = 1e-5);
}
