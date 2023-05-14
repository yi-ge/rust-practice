use rust_practice::sort::distant_barcodes::Solution;

#[test]
fn rearrange_barcodes() {
    // 示例 1：
    // 输入：barcodes = [1,1,1,2,2,2]
    // 输出：[2,1,2,1,2,1]
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    let res = vec![2, 1, 2, 1, 2, 1];
    assert_eq!(Solution::rearrange_barcodes(barcodes), res);

    // 示例 2：
    // 输入：barcodes = [1,1,1,1,2,2,3,3]
    // 输出：[1,3,1,3,2,1,2,1]
    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let res = vec![1, 3, 1, 2, 1, 3, 2, 1]; // 可能有多个正确的值
    assert_eq!(Solution::rearrange_barcodes(barcodes), res);
}
