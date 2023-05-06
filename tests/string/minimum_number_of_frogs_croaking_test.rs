use rust_practice::string::minimum_number_of_frogs_croaking::Solution;

#[test]
fn min_number_of_frogs() {
    // 示例 1：
    // 输入：croakOfFrogs = "croakcroak"
    // 输出：1
    // 解释：一只青蛙 “呱呱” 两次
    let croak_of_frogs = "croakcroak".to_string();
    assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), 1);

    // 示例 2：
    // 输入：croakOfFrogs = "crcoakroak"
    // 输出：2
    // 解释：最少需要两只青蛙，“呱呱” 声用黑体标注
    // 第一只青蛙 "crcoakroak"
    // 第二只青蛙 "crcoakroak"
    let croak_of_frogs = "crcoakroak".to_string();
    assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), 2);

    // 示例 3：
    // 输入：croakOfFrogs = "croakcrook"
    // 输出：-1
    // 解释：给出的字符串不是 "croak" 的有效组合。
    let croak_of_frogs = "croakcrook".to_string();
    assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), -1);
}
