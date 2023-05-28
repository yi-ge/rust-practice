use rust_practice::array::find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows::Solution;

#[test]
fn kth_smallest() {
    // 示例 1：
    // 输入：mat = [[1,3,11],[2,4,6]], k = 5
    // 输出：7
    // 解释：从每一行中选出一个元素，前 k 个和最小的数组分别是：
    // [1,2], [1,4], [3,2], [3,4], [1,6]。其中第 5 个的和是 7 。
    // 示例 2：
    // 输入：mat = [[1,3,11],[2,4,6]], k = 9
    // 输出：17
    let mat = vec![vec![1,3,11],vec![2,4,6]];
    let k = 5;
    assert_eq!(Solution::kth_smallest(mat, k), 7);

    // 示例 3：
    // 输入：mat = [[1,10,10],[1,4,5],[2,3,6]], k = 7
    // 输出：9
    // 解释：从每一行中选出一个元素，前 k 个和最小的数组分别是：
    // [1,1,2], [1,1,3], [1,4,2], [1,4,3], [1,1,6], [1,5,2], [1,5,3]。其中第 7 个的和是 9 。
    let mat = vec![vec![1,10,10],vec![1,4,5],vec![2,3,6]];
    let k = 7;
    assert_eq!(Solution::kth_smallest(mat, k), 9);

    // 示例 4：
    // 输入：mat = [[1,1,10],[2,2,9]], k = 7
    // 输出：12
    let mat = vec![vec![1,1,10],vec![2,2,9]];
    let k = 7;
    assert_eq!(Solution::kth_smallest(mat, k), 12);
}
