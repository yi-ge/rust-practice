use rust_practice::map::powerful_integers::Solution;
use std::collections::HashSet;

#[test]
fn powerful_integers() {
    // 示例 1：
    // 输入：x = 2, y = 3, bound = 10
    // 输出：[2,3,4,5,7,9,10]
    // 解释：
    // 2 = 20 + 30
    // 3 = 21 + 30
    // 4 = 20 + 31
    // 5 = 21 + 31
    // 7 = 22 + 31
    // 9 = 23 + 30
    // 10 = 20 + 32
    let x = 2;
    let y = 3;
    let bound = 10;
    let result = vec![2, 3, 4, 5, 7, 9, 10]
        .into_iter()
        .collect::<HashSet<_>>();
    let output = Solution::powerful_integers(x, y, bound)
        .into_iter()
        .collect::<HashSet<_>>();
    assert_eq!(output, result);

    // 示例 2：
    // 输入：x = 3, y = 5, bound = 15
    // 输出：[2,4,6,8,10,14]
    let x = 3;
    let y = 5;
    let bound = 15;
    let result = vec![2, 4, 6, 8, 10, 14].into_iter().collect::<HashSet<_>>();
    let output = Solution::powerful_integers(x, y, bound)
        .into_iter()
        .collect::<HashSet<_>>();
    assert_eq!(output, result);
}
