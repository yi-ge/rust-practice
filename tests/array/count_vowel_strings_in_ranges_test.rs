use rust_practice::array::count_vowel_strings_in_ranges::Solution;

#[test]
fn vowel_strings() {
    // 示例 1：
    // 输入：words = ["aba","bcb","ece","aa","e"], queries = [[0,2],[1,4],[1,1]]
    // 输出：[2,3,0]
    // 解释：以元音开头和结尾的字符串是 "aba"、"ece"、"aa" 和 "e" 。
    // 查询 [0,2] 结果为 2（字符串 "aba" 和 "ece"）。
    // 查询 [1,4] 结果为 3（字符串 "ece"、"aa"、"e"）。
    // 查询 [1,1] 结果为 0 。
    // 返回结果 [2,3,0] 。
    let words = vec![
        "aba".to_string(),
        "bcb".to_string(),
        "ece".to_string(),
        "aa".to_string(),
        "e".to_string(),
    ];
    let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
    assert_eq!(Solution::vowel_strings(words, queries), vec![2, 3, 0]);

    // 示例 2：
    // 输入：words = ["a","e","i"], queries = [[0,2],[0,1],[2,2]]
    // 输出：[3,2,1]
    // 解释：每个字符串都满足这一条件，所以返回 [3,2,1] 。
    let words = vec!["a".to_string(), "e".to_string(), "i".to_string()];
    let queries = vec![vec![0, 2], vec![0, 1], vec![2, 2]];
    assert_eq!(Solution::vowel_strings(words, queries), vec![3, 2, 1]);
}
