// 对数组执行操作
// https://leetcode.cn/problems/apply-operations-to-an-array
// INLINE  ../../images/array/apply_operations_to_an_array.jpeg

pub struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut j = 0; // j 是下一个非零元素的索引
        for i in 0..n {
            // i 是当前元素的索引
            // 如果当前元素不是最后一个元素，且它等于下一个元素
            if i + 1 < n && nums[i] == nums[i + 1] {
                // 将当前元素翻倍
                nums[i] *= 2;
                // 将下一个元素设置为 0
                nums[i + 1] = 0;
            }
            // 如果当前元素不是零
            if nums[i] != 0 {
                // 将它与索引 j 的元素交换（下一个非零元素）
                nums.swap(i, j);
                // 增加 j
                j += 1;
            }
        }
        nums
    }
}
