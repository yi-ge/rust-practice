use rust_practice::array::store_water::Solution;

#[test]
fn store_water() {
    // 示例 1：
    // 输入：bucket = [1,3], vat = [6,8]
    // 输出：4
    // 解释：
    // 第 1 次操作升级 bucket[0]；
    // 第 2 ~ 4 次操作均选择蓄水，即可完成蓄水要求。
    let bucket = vec![1, 3];
    let vat = vec![6, 8];
    assert_eq!(Solution::store_water(bucket, vat), 4);

    // 示例 2：
    // 输入：bucket = [9,0,1], vat = [0,2,2]
    // 输出：3
    // 解释：
    // 第 1 次操作均选择升级 bucket[1]
    // 第 2~3 次操作选择蓄水，即可完成蓄水要求。
    let bucket = vec![9, 0, 1];
    let vat = vec![0, 2, 2];
    assert_eq!(Solution::store_water(bucket, vat), 3);
}
