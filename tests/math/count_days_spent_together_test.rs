use rust_practice::math::count_days_spent_together::Solution;

#[test]
fn count_days_together() {
    // 示例 1：
    // 输入：arriveAlice = "08-15", leaveAlice = "08-18", arriveBob = "08-16", leaveBob = "08-19"
    // 输出：3
    // 解释：Alice 从 8 月 15 号到 8 月 18 号在罗马。Bob 从 8 月 16 号到 8 月 19 号在罗马，他们同时在罗马的日期为 8 月 16、17 和 18 号。所以答案为 3 。
    assert_eq!(
        Solution::count_days_together(
            "08-15".to_string(),
            "08-18".to_string(),
            "08-16".to_string(),
            "08-19".to_string()
        ),
        3
    );

    // 示例 2：
    // 输入：arriveAlice = "10-01", leaveAlice = "10-31", arriveBob = "11-01", leaveBob = "12-31"
    // 输出：0
    // 解释：Alice 和 Bob 没有同时在罗马的日子，所以我们返回 0 。
    assert_eq!(
        Solution::count_days_together(
            "10-01".to_string(),
            "10-31".to_string(),
            "11-01".to_string(),
            "12-31".to_string()
        ),
        0
    );
}
