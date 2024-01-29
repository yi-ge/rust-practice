use rust_practice::search::freedom_trail::Solution;

#[test]
fn find_rotate_steps_test() {
    // 示例 1：
    // 输入: ring = "godding", key = "gd"
    // 输出: 4
    // 解释:
    //  对于 key 的第一个字符 'g'，已经在正确的位置, 我们只需要1步来拼写这个字符。
    //  对于 key 的第二个字符 'd'，我们需要逆时针旋转 ring "godding" 2步使它变成 "ddinggo"。
    //  当然, 我们还需要1步进行拼写。
    //  因此最终的输出是 4。
    let ring = "godding".to_string();
    let key = "gd".to_string();
    assert_eq!(Solution::find_rotate_steps(ring, key), 4);

    // 示例 2:
    // 输入: ring = "godding", key = "godding"
    // 输出: 13
    let ring = "godding".to_string();
    let key = "godding".to_string();
    assert_eq!(Solution::find_rotate_steps(ring, key), 13);
}
