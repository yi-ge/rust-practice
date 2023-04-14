use rust_practice::tree::camelcase_matching::Solution;

#[test]
fn camel_match() {
    // 示例 1：
    // 输入：queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FB"
    // 输出：[true,false,true,true,false]
    // 示例：
    // "FooBar" 可以这样生成："F" + "oo" + "B" + "ar"。
    // "FootBall" 可以这样生成："F" + "oot" + "B" + "all".
    // "FrameBuffer" 可以这样生成："F" + "rame" + "B" + "uffer".
    let queries = vec![
        "FooBar".to_string(),
        "FooBarTest".to_string(),
        "FootBall".to_string(),
        "FrameBuffer".to_string(),
        "ForceFeedBack".to_string(),
    ];
    let pattern = "FB".to_string();
    assert_eq!(
        Solution::camel_match(queries, pattern),
        vec![true, false, true, true, false]
    );

    // 示例：
    // 示例 2：
    // 示例 2：
    // 输入：queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBa"
    // 输出：[true,false,true,false,false]
    // 解释：
    // "FooBar" 可以这样生成："Fo" + "o" + "Ba" + "r".
    // "FootBall" 可以这样生成："Fo" + "ot" + "Ba" + "ll".
    let queries = vec![
        "FooBar".to_string(),
        "FooBarTest".to_string(),
        "FootBall".to_string(),
        "FrameBuffer".to_string(),
        "ForceFeedBack".to_string(),
    ];
    let pattern = "FoBa".to_string();
    assert_eq!(
        Solution::camel_match(queries, pattern),
        vec![true, false, true, false, false]
    );

    // 示例 3：
    // 输出：queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBaT"
    // 输入：[false,true,false,false,false]
    // 解释：
    // "FooBarTest" 可以这样生成："Fo" + "o" + "Ba" + "r" + "T" + "est".
    let queries = vec![
        "FooBar".to_string(),
        "FooBarTest".to_string(),
        "FootBall".to_string(),
        "FrameBuffer".to_string(),
        "ForceFeedBack".to_string(),
    ];
    let pattern = "FoBaT".to_string();
    assert_eq!(
        Solution::camel_match(queries, pattern),
        vec![false, true, false, false, false]
    );
}
