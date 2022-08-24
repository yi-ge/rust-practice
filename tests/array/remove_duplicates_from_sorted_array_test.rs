use rust_practice::array::remove_duplicates_from_sorted_array::Solution;

#[test]
fn remove_duplicates() {
    // 示例 1：
    // 输入：nums = [1,1,2]
    // 输出：2, nums = [1,2,_]
    // 解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    assert_eq!(nums[0], 1);
    assert_eq!(nums[1], 2);

    // 示例 2：
    // 输入：nums = [0,0,1,1,1,2,2,3,3,4]
    // 输出：5, nums = [0,1,2,3,4]
    // 解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    assert_eq!(&nums[..5], &[0, 1, 2, 3, 4][..]);
}
