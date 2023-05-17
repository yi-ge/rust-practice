use rust_practice::array::determine_if_two_events_have_conflict::Solution;

#[test]
fn have_conflict() {
    // 示例 1：
    // 输入：event1 = ["01:15","02:00"], event2 = ["02:00","03:00"]
    // 输出：true
    // 解释：两个事件在 2:00 出现交集。
    let event1 = vec!["01:15".to_string(), "02:00".to_string()];
    let event2 = vec!["02:00".to_string(), "03:00".to_string()];
    assert_eq!(Solution::have_conflict(event1, event2), true);

    // 示例 2：
    // 输入：event1 = ["01:00","02:00"], event2 = ["01:20","03:00"]
    // 输出：true
    // 解释：两个事件的交集从 01:20 开始，到 02:00 结束。
    let event1 = vec!["01:00".to_string(), "02:00".to_string()];
    let event2 = vec!["01:20".to_string(), "03:00".to_string()];
    assert_eq!(Solution::have_conflict(event1, event2), true);

    // 示例 3：
    // 输入：event1 = ["10:00","11:00"], event2 = ["14:00","15:00"]
    // 输出：false
    // 解释：两个事件不存在交集。
    let event1 = vec!["10:00".to_string(), "11:00".to_string()];
    let event2 = vec!["14:00".to_string(), "15:00".to_string()];
    assert_eq!(Solution::have_conflict(event1, event2), false);
}
