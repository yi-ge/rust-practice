use rust_practice::sort::largest_values_from_labels::Solution;

#[test]
fn largest_vals_from_labels() {
    // 示例 1：
    // 输入：values = [5,4,3,2,1], labels = [1,1,2,2,3], numWanted = 3, useLimit = 1
    // 输出：9
    // 解释：选出的子集是第一项，第三项和第五项。
    let values = vec![5, 4, 3, 2, 1];
    let labels = vec![1, 1, 2, 2, 3];
    let num_wanted = 3;
    let use_limit = 1;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        9
    );

    // 示例 2：
    // 输入：values = [5,4,3,2,1], labels = [1,3,3,3,2], numWanted = 3, useLimit = 2
    // 输出：12
    // 解释：选出的子集是第一项，第二项和第三项。
    let values = vec![5, 4, 3, 2, 1];
    let labels = vec![1, 3, 3, 3, 2];
    let num_wanted = 3;
    let use_limit = 2;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        12
    );

    // 示例 3：
    // 输入：values = [9,8,8,7,6], labels = [0,0,0,1,1], numWanted = 3, useLimit = 1
    // 输出：16
    // 解释：选出的子集是第一项和第四项。
    let values = vec![9, 8, 8, 7, 6];
    let labels = vec![0, 0, 0, 1, 1];
    let num_wanted = 3;
    let use_limit = 1;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        16
    );
}
