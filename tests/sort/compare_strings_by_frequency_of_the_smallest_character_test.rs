use rust_practice::sort::compare_strings_by_frequency_of_the_smallest_character::Solution;

#[test]
fn num_smaller_by_frequency() {
    // 示例 1：
    // 输入：queries = ["cbd"], words = ["zaaaz"]
    // 输出：[1]
    // 解释：查询 f("cbd") = 1，而 f("zaaaz") = 3 所以 f("cbd") < f("zaaaz")。
    let queries = vec!["cbd".to_string()];
    let words = vec!["zaaaz".to_string()];
    let result = Solution::num_smaller_by_frequency(queries, words);
    assert_eq!(result, vec![1]);

    // 示例 2：
    // 输入：queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
    // 输出：[1,2]
    // 解释：第一个查询 f("bbb") < f("aaaa")，第二个查询 f("aaa") 和 f("aaaa") 都 > f("cc")。
    let queries = vec!["bbb".to_string(), "cc".to_string()];
    let words = vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string()];
    let result = Solution::num_smaller_by_frequency(queries, words);
    assert_eq!(result, vec![1, 2]);
}
