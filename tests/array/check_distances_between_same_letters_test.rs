use rust_practice::array::check_distances_between_same_letters::Solution;

#[test]
fn check_distances() {
    // 示例 1：
    // 输入：s = "abaccb", distance = [1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    // 输出：true
    // 解释：
    // - 'a' 在下标 0 和下标 2 处出现，所以满足 distance[0] = 1 。
    // - 'b' 在下标 1 和下标 5 处出现，所以满足 distance[1] = 3 。
    // - 'c' 在下标 3 和下标 4 处出现，所以满足 distance[2] = 0 。
    // 注意 distance[3] = 5 ，但是由于 'd' 没有在 s 中出现，可以忽略。
    // 因为 s 是一个匀整字符串，返回 true 。
    let s1 = "abaccb";
    let distance1 = vec![
        1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(Solution::check_distances(s1.to_string(), distance1), true);

    // 示例 2：
    // 输入：s = "aa", distance = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    // 输出：false
    // 解释：
    // - 'a' 在下标 0 和 1 处出现，所以两次出现之间的字母数量为 0 。
    // 但是 distance[0] = 1 ，s 不是一个匀整字符串。
    let s2 = "aa";
    let distance2 = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(Solution::check_distances(s2.to_string(), distance2), false);

    // 测试 3：
    let s3 = "abbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzza";
    let distance3 = vec![
        49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(Solution::check_distances(s3.to_string(), distance3), false);
}
