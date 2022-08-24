use rust_practice::string::check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence::Solution;

#[test]
fn is_prefix_of_word() {
    // 示例 1：
    // 输入：sentence = "i love eating burger", searchWord = "burg"
    // 输出：4
    // 解释："burg" 是 "burger" 的前缀，而 "burger" 是句子中第 4 个单词。
    assert_eq!(Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()), 4);

    // 示例 2：
    // 输入：sentence = "this problem is an easy problem", searchWord = "pro"
    // 输出：2
    // 解释："pro" 是 "problem" 的前缀，而 "problem" 是句子中第 2 个也是第 6 个单词，但是应该返回最小下标 2 。
    assert_eq!(Solution::is_prefix_of_word("this problem is an easy problem".to_string(), "pro".to_string()), 2);

    // 示例 3：
    // 输入：sentence = "i am tired", searchWord = "you"
    // 输出：-1
    // 解释："you" 不是句子中任何单词的前缀。
    assert_eq!(Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()), -1);
}
