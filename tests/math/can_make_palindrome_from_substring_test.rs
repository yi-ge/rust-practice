use rust_practice::math::can_make_palindrome_from_substring::Solution;

#[test]
fn can_make_pali_queries() {
    // 示例：
    // 输入：s = "abcda", queries = [[3,3,0],[1,2,0],[0,3,1],[0,3,2],[0,4,1]]
    // 输出：[true,false,false,true,true]
    // 解释：
    // queries[0] : 子串 = "d"，回文。
    // queries[1] : 子串 = "bc"，不是回文。
    // queries[2] : 子串 = "abcd"，只替换 1 个字符是变不成回文串的。
    // queries[3] : 子串 = "abcd"，可以变成回文的 "abba"。 也可以变成 "baab"，先重新排序变成 "bacd"，然后把 "cd" 替换为 "ab"。
    // queries[4] : 子串 = "abcda"，可以变成回文的 "abcba"。
    let s = "abcda".to_string();
    let queries = vec![
        vec![3, 3, 0],
        vec![1, 2, 0],
        vec![0, 3, 1],
        vec![0, 3, 2],
        vec![0, 4, 1],
    ];
    let result = Solution::can_make_pali_queries(s, queries);
    assert_eq!(result, vec![true, false, false, true, true]);
}
