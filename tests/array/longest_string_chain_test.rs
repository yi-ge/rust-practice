use rust_practice::array::longest_string_chain::Solution;

#[test]
fn longest_str_chain() {
    // 示例 1：
    // 输入：words = ["a","b","ba","bca","bda","bdca"]
    // 输出：4
    // 解释：最长单词链之一为 ["a","ba","bda","bdca"]
    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "ba".to_string(),
        "bca".to_string(),
        "bda".to_string(),
        "bdca".to_string(),
    ];
    assert_eq!(Solution::longest_str_chain(words), 4);

    // 示例 2:
    // 输入：words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
    // 输出：5
    // 解释：所有的单词都可以放入单词链 ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].
    let words = vec![
        "xbc".to_string(),
        "pcxbcf".to_string(),
        "xb".to_string(),
        "cxbc".to_string(),
        "pcxbc".to_string(),
    ];
    assert_eq!(Solution::longest_str_chain(words), 5);

    // 示例 3:
    // 输入：words = ["abcd","dbqca"]
    // 输出：1
    // 解释：字链["abcd"]是最长的字链之一。
    // ["abcd"，"dbqca"]不是一个有效的单词链，因为字母的顺序被改变了。
    let words = vec!["abcd".to_string(), "dbqca".to_string()];
    assert_eq!(Solution::longest_str_chain(words), 1);
}
