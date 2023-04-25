use rust_practice::sort::sort_the_people::Solution;

#[test]
fn sort_people() {
    // 示例 1：
    // 输入：names = ["Mary","John","Emma"], heights = [180,165,170]
    // 输出：["Mary","Emma","John"]
    // 解释：Mary 最高，接着是 Emma 和 John 。
    let name = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
    let heights = vec![180, 165, 170];
    let result = Solution::sort_people(name, heights);
    assert_eq!(
        result,
        vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
    );

    // 示例 2：
    // 输入：names = ["Alice","Bob","Bob"], heights = [155,185,150]
    // 输出：["Bob","Alice","Bob"]
    // 解释：第一个 Bob 最高，然后是 Alice 和第二个 Bob 。
    let name = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
    let heights = vec![155, 185, 150];
    let result = Solution::sort_people(name, heights);
    assert_eq!(
        result,
        vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
    );
}
