use rust_practice::array::count_good_triplets::Solution;

#[test]
fn count_good_triplets() {
    // 示例 1：
    // 输入：arr = [3,0,1,1,9,7], a = 7, b = 2, c = 3
    // 输出：4
    // 解释：一共有 4 个好三元组：[(3,0,1), (3,0,1), (3,1,1), (0,1,1)] 。
    let arr = vec![3, 0, 1, 1, 9, 7];
    assert_eq!(Solution::count_good_triplets(arr, 7, 2, 3), 4);

    // 示例 2：
    // 输入：arr = [1,1,2,2,3], a = 0, b = 0, c = 1
    // 输出：0
    // 解释：不存在满足所有条件的三元组。
    let arr = vec![1, 1, 2, 2, 3];
    assert_eq!(Solution::count_good_triplets(arr, 0, 0, 1), 0);
}
