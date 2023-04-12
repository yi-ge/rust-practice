use rust_practice::string::longest_chunked_palindrome_decomposition::Solution;

#[test]
fn longest_decomposition() {
    // 示例 1：
    // 输入：text = "ghiabcdefhelloadamhelloabcdefghi"
    // 输出：7
    // 解释：我们可以把字符串拆分成 "(ghi)(abcdef)(hello)(adam)(hello)(abcdef)(ghi)"。
    let text = "ghiabcdefhelloadamhelloabcdefghi".to_string();
    assert_eq!(Solution::longest_decomposition(text), 7);

    // 示例 2：
    // 输入：text = "merchant"
    // 输出：1
    // 解释：我们可以把字符串拆分成 "(merchant)"。
    let text = "merchant".to_string();
    assert_eq!(Solution::longest_decomposition(text), 1);

    // 示例 3：
    // 输入：text = "antaprezatepzapreanta"
    // 输出：11
    // 解释：我们可以把字符串拆分成 "(a)(nt)(a)(pre)(za)(tpe)(za)(pre)(a)(nt)(a)"。
    let text = "antaprezatepzapreanta".to_string();
    assert_eq!(Solution::longest_decomposition(text), 11);

    // 示例 4：
    let text = "".to_string();
    assert_eq!(Solution::longest_decomposition(text), 0);
}
