// 删除有序数组中的重复项
// https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
// INLINE  ../../images/array/remove_duplicates_from_sorted_array.jpeg
// 解题思路：双指针。

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // 若数组为空，则返回0
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0; // 慢指针
        for j in 1..nums.len() {
            // 快指针
            if nums[i] != nums[j] {
                // 若快指针指向的数值不等于慢指针指向的数值
                if j - i > 1 {
                    // 若快指针和慢指针之间有重复的数值
                    nums[i + 1] = nums[j]; // 将重复的数值置于慢指针的下一位
                }
                i += 1; // 慢指针移动一位
            }
        }

        (i + 1) as i32 // 返回去重后数组的长度
    }
}
