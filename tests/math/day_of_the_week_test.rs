use rust_practice::math::day_of_the_week::Solution;

#[test]
fn day_of_the_week() {
    // 示例 1：
    // 输入：day = 31, month = 8, year = 2019
    // 输出："Saturday"
    let day = 31;
    let month = 8;
    let year = 2019;
    let result = Solution::day_of_the_week(day, month, year);
    assert_eq!(result, "Saturday");

    // 示例 2：
    // 输入：day = 18, month = 7, year = 1999
    // 输出："Sunday"
    let day = 18;
    let month = 7;
    let year = 1999;
    let result = Solution::day_of_the_week(day, month, year);
    assert_eq!(result, "Sunday");

    // 示例 3：
    // 输入：day = 15, month = 8, year = 1993
    // 输出："Sunday"
    let day = 15;
    let month = 8;
    let year = 1993;
    let result = Solution::day_of_the_week(day, month, year);
    assert_eq!(result, "Sunday");
}
