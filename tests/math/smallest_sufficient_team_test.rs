use rust_practice::math::smallest_sufficient_team::Solution;

#[test]
fn test_smallest_sufficient_team() {
    // 示例 1：
    // 输入：req_skills = ["java","nodejs","reactjs"], people = [["java"],["nodejs"],["nodejs","reactjs"]]
    // 输出：[0,2]
    let req_skills1 = vec![
        "java".to_string(),
        "nodejs".to_string(),
        "reactjs".to_string(),
    ];
    let people1 = vec![
        vec!["java".to_string()],
        vec!["nodejs".to_string()],
        vec!["nodejs".to_string(), "reactjs".to_string()],
    ];
    let expected_output1 = vec![2, 0];
    assert_eq!(
        Solution::smallest_sufficient_team(req_skills1, people1),
        expected_output1
    );

    // 示例 2：
    // 输入：req_skills = ["algorithms","math","java","reactjs","csharp","aws"], people = [["algorithms","math","java"],["algorithms","math","reactjs"],["java","csharp","aws"],["reactjs","csharp"],["csharp","math"],["aws","java"]]
    // 输出：[1,2]
    let req_skills2 = vec![
        "algorithms".to_string(),
        "math".to_string(),
        "java".to_string(),
        "reactjs".to_string(),
        "csharp".to_string(),
        "aws".to_string(),
    ];
    let people2 = vec![
        vec![
            "algorithms".to_string(),
            "math".to_string(),
            "java".to_string(),
        ],
        vec![
            "algorithms".to_string(),
            "math".to_string(),
            "reactjs".to_string(),
        ],
        vec!["java".to_string(), "csharp".to_string(), "aws".to_string()],
        vec!["reactjs".to_string(), "csharp".to_string()],
        vec!["csharp".to_string(), "math".to_string()],
        vec!["aws".to_string(), "java".to_string()],
    ];
    let expected_output2 = vec![2, 1];
    assert_eq!(
        Solution::smallest_sufficient_team(req_skills2, people2),
        expected_output2
    );
}
