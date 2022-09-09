use rust_practice::stack::crawler_log_folder::Solution;

#[test]
fn min_operations() {
    // 示例 1：
    // 输入：logs = ["d1/","d2/","../","d21/","./"]
    // 输出：2
    // 解释：执行 "../" 操作变更文件夹 2 次，即可回到主文件夹
    let logs = vec![
        "d1/".to_string(),
        "d2/".to_string(),
        "../".to_string(),
        "d21/".to_string(),
        "./".to_string(),
    ];
    assert_eq!(Solution::min_operations(logs), 2);

    // 示例 2：
    // 输入：logs = ["d1/","d2/","./","d3/","../","d31/"]
    // 输出：3
    let logs = vec![
        "d1/".to_string(),
        "d2/".to_string(),
        "d3/".to_string(),
        "../".to_string(),
        "d31/".to_string(),
    ];
    assert_eq!(Solution::min_operations(logs), 3);

    // 示例 3：
    // 输入：logs = ["d1/","../","../","../"]
    // 输出：0
    let logs = vec![
        "d1/".to_string(),
        "../".to_string(),
        "../".to_string(),
        "../".to_string(),
    ];
    assert_eq!(Solution::min_operations(logs), 0);

    // 示例 4：
    // 输入：logs = ["./","wz4/","../","mj2/","../","../","ik0/","il7/"]
    // 输出：2
    let logs = vec![
        "./".to_string(),
        "wz4/".to_string(),
        "../".to_string(),
        "mj2/".to_string(),
        "../".to_string(),
        "../".to_string(),
        "ik0/".to_string(),
        "il7/".to_string(),
    ];
    assert_eq!(Solution::min_operations(logs), 2);
}
