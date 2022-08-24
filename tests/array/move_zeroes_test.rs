use rust_practice::array::move_zeroes::Solution;

#[test]
fn move_zeroes() {
    // 示例 1:
    // 输入: nums = [0,1,0,3,12]
    // 输出: [1,3,12,0,0]
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(&nums[..], &[1, 3, 12, 0, 0][..]);

    // 示例 2:
    // 输入: nums = [0]
    // 输出: [0]
    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(&nums[..], &[0][..]);
}
