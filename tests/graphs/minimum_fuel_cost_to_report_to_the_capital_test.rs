use rust_practice::graphs::minimum_fuel_cost_to_report_to_the_capital::Solution;

#[test]
fn minimum_fuel_cost() {
    // 示例 1：
    // 输入：roads = [[0,1],[0,2],[0,3]], seats = 5
    // 输出：3
    // 解释：
    // - 代表 1 直接到达首都，消耗 1 升汽油。
    // - 代表 2 直接到达首都，消耗 1 升汽油。
    // - 代表 3 直接到达首都，消耗 1 升汽油。
    // 最少消耗 3 升汽油。

    let roads1 = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
    let seats1 = 5;
    assert_eq!(Solution::minimum_fuel_cost(roads1, seats1), 3);

    // 示例 2：
    // 输入：roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2
    // 输出：7
    // 解释：
    // - 代表 2 到达城市 3 ，消耗 1 升汽油。
    // - 代表 2 和代表 3 一起到达城市 1 ，消耗 1 升汽油。
    // - 代表 2 和代表 3 一起到达首都，消耗 1 升汽油。
    // - 代表 1 直接到达首都，消耗 1 升汽油。
    // - 代表 5 直接到达首都，消耗 1 升汽油。
    // - 代表 6 到达城市 4 ，消耗 1 升汽油。
    // - 代表 4 和代表 6 一起到达首都，消耗 1 升汽油。
    // 最少消耗 7 升汽油。

    let roads2 = vec![
        vec![3, 1],
        vec![3, 2],
        vec![1, 0],
        vec![0, 4],
        vec![0, 5],
        vec![4, 6],
    ];
    let seats2 = 2;
    assert_eq!(Solution::minimum_fuel_cost(roads2, seats2), 7);

    // 示例 3：
    // 输入：roads = [], seats = 1
    // 输出：0
    // 解释：没有代表需要从别的城市到达首都。

    let roads3 = vec![];
    let seats3 = 1;
    assert_eq!(Solution::minimum_fuel_cost(roads3, seats3), 0);
}
