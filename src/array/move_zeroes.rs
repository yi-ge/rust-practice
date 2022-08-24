// 移动零
// https://leetcode.cn/problems/move-zeroes/
// INLINE  ../../images/array/move_zeroes.jpeg

pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                // 将非0元素按顺序放入数组0到i-1的位置
                nums[i] = nums[j];
                i += 1;
            }
        }

        // 剩余部分全部置0
        for k in i..nums.len() {
            nums[k] = 0;
        }
    }
}
