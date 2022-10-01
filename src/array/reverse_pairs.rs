// 翻转对
// https://leetcode.cn/problems/reverse-pairs/
// INLINE  ../../images/array/reverse_pairs.jpeg
// 解题思路：基于并归排序作计数。

pub struct Solution;

impl Solution {
    fn merge(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
        let mut i = left;
        let mut j = middle + 1;
        let mut k = left;

        let mut temp = vec![];

        while k <= right {
            if i > middle {
                temp.push(nums[j]);
                j += 1;
                k += 1;
            } else if j > right {
                temp.push(nums[i]);
                i += 1;
                k += 1;
            } else if nums[i] < nums[j] {
                temp.push(nums[i]);
                i += 1;
                k += 1;
            } else {
                temp.push(nums[j]);
                j += 1;
                k += 1;
            }
        }

        for i in 0..=(right - left) {
            nums[left + i] = temp[i];
        }
    }

    fn merge_sort_recursion_count(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        if left >= right {
            return 0;
        }

        let middle = (left + right) >> 1;
        // 先统计左、右子数组的重要翻转对数量
        let mut count = Self::merge_sort_recursion_count(nums, left, middle)
            + Self::merge_sort_recursion_count(nums, middle + 1, right);

        // 再统计左、右子数组之间的重要翻转对数量
        let mut i = left;
        let mut j = middle + 1;
        while i <= middle && j <= right {
            // 防止溢出
            if nums[i] as i64 > 2 * nums[j] as i64 {
                count += middle - i + 1;
                j += 1;
            } else {
                i += 1;
            }
        }

        Self::merge(nums, left, middle, right);
        count
    }

    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return 0;
        }

        Self::merge_sort_recursion_count(&mut nums, 0, len - 1) as i32
    }
}
