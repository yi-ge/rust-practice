use rust_practice::array::group_anagrams::Solution;

#[test]
fn group_anagrams() {
    // 示例 1:
    // 输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
    // 输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    let mut res = Solution::group_anagrams(strs);
    res.sort_by(|a, b| a.to_vec().len().cmp(&b.to_vec().len()));

    for i in 0..res.len() {
        res[i].sort();
        // print!("{:?}", res[i]);
    }

    let mut ans = vec![
        vec!["bat".to_string()],
        vec!["nat".to_string(), "tan".to_string()],
        vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
    ];
    for i in 0..ans.len() {
        ans[i].sort();
    }

    assert_eq!(res, ans);

    // 示例 2:
    // 输入: strs = [""]
    // 输出: [[""]]
    let strs = vec!["".to_string()];
    assert_eq!(Solution::group_anagrams(strs), vec![vec!["".to_string()]]);

    // 示例 3:
    // 输入: strs = ["a"]
    // 输出: [["a"]]
    let strs = vec!["a".to_string()];
    assert_eq!(Solution::group_anagrams(strs), vec![vec!["a".to_string()]]);
}
