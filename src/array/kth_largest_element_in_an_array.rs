// 数组中的第K个最大元素
// https://leetcode.cn/problems/kth-largest-element-in-an-array/
// INLINE  ../../images/array/kth_largest_element_in_an_array.jpeg
// 解题思路：题目要求设计时间复杂度为O(n)的算法。因此可以考虑基于快速排序来实现。

use rand::Rng;

pub struct Solution;

impl Solution {
    fn partition(nums: &mut Vec<i32>, left: usize, right: usize, pivot_index: usize) -> usize {
        let pivot = nums[pivot_index];
        nums.swap(pivot_index, right);
        let mut store_index = left;

        for i in left..right {
            if nums[i] <= pivot {
                nums.swap(store_index, i);
                store_index += 1;
            }
        }

        nums.swap(store_index, right);
        store_index
    }

    fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k_smallest: usize) -> i32 {
        if left == right {
            return nums[left];
        }
        let mut rng = rand::thread_rng();
        let mut pivot_index = left + rng.gen_range(0..right - left);
        pivot_index = Self::partition(nums, left, right, pivot_index);
        return if k_smallest == pivot_index {
            nums[k_smallest]
        } else if k_smallest < pivot_index {
            Self::quick_select(nums, left, pivot_index - 1, k_smallest)
        } else {
            Self::quick_select(nums, pivot_index + 1, right, k_smallest)
        };
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        Self::quick_select(&mut nums, 0, len - 1, len - k as usize)
    }
}
