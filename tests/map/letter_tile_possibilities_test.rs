use rust_practice::map::letter_tile_possibilities::Solution;

#[test]
fn num_tile_possibilities() {
    // 示例 1：
    // 输入："AAB"
    // 输出：8
    // 解释：可能的序列为 "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA"。
    let tiles = "AAB".to_string();
    assert_eq!(Solution::num_tile_possibilities(tiles), 8);

    // 示例 2：
    // 输入："AAABBC"
    // 输出：188
    let tiles = "AAABBC".to_string();
    assert_eq!(Solution::num_tile_possibilities(tiles), 188);

    // 示例 3：
    // 输入："V"
    // 输出：1
    let tiles = "V".to_string();
    assert_eq!(Solution::num_tile_possibilities(tiles), 1);
}
