use rust_practice::array::online_majority_element_in_subarray::MajorityChecker;

#[test]
fn majority_checker() {
    // 示例 1：
    // 输入:
    // ["MajorityChecker", "query", "query", "query"]
    // [[[1, 1, 2, 2, 1, 1]], [0, 5, 4], [0, 3, 3], [2, 3, 2]]
    // 输出：
    // [null, 1, -1, 2]

    // 解释：
    // MajorityChecker majorityChecker = new MajorityChecker([1,1,2,2,1,1]);
    // majorityChecker.query(0,5,4); // 返回 1
    // majorityChecker.query(0,3,3); // 返回 -1
    // majorityChecker.query(2,3,2); // 返回 2
    // Test case 1
    let arr = vec![1, 1, 2, 2, 1, 1];
    let mut obj = MajorityChecker::new(arr);
    assert_eq!(obj.query(0, 5, 4), 1);
    assert_eq!(obj.query(0, 3, 3), -1);
    assert_eq!(obj.query(2, 3, 2), 2);

    // Test case 2: No majority element
    let arr = vec![1, 2, 3, 4, 5, 6];
    let mut obj = MajorityChecker::new(arr);
    assert_eq!(obj.query(0, 5, 4), -1);
    assert_eq!(obj.query(0, 3, 3), -1);

    // Test case 3: Multiple majority elements
    let arr = vec![3, 3, 3, 2, 2, 2, 1, 1, 1];
    let mut obj = MajorityChecker::new(arr);
    // assert_eq!(obj.query(0, 8, 3), 1);
    assert_eq!(obj.query(0, 2, 2), 3);
    assert_eq!(obj.query(3, 5, 2), 2);
    assert_eq!(obj.query(6, 8, 2), 1);

    // Test case 4: Subarray with all elements the same
    let arr = vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4];
    let mut obj = MajorityChecker::new(arr);
    assert_eq!(obj.query(0, 9, 5), 4);
    assert_eq!(obj.query(1, 8, 4), 4);

    // Test case 5: Complex subarray
    let arr = vec![2, 1, 3, 1, 1, 3, 3, 3, 2, 1, 1, 1, 3, 3, 3, 2];
    let mut obj = MajorityChecker::new(arr);
    // assert_eq!(obj.query(0, 15, 6), 3);
    assert_eq!(obj.query(1, 10, 5), 1);
    assert_eq!(obj.query(5, 14, 5), 3);
}
