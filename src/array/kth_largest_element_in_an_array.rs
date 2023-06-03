// 数组中的第K个最大元素
// https://leetcode.cn/problems/kth-largest-element-in-an-array/
// INLINE  ../../images/array/kth_largest_element_in_an_array.jpeg
// 解题思路：题目要求设计时间复杂度为O(n)的算法。因此可以考虑基于快速排序来实现。

use rand::Rng;

pub struct Solution;

impl Solution {
    // 划分函数，根据基准值将数组划分成左右两部分，返回基准值的位置
    fn partition(nums: &mut Vec<i32>, left: usize, right: usize, pivot_index: usize) -> usize {
        let pivot = nums[pivot_index]; // 选择基准值
        nums.swap(pivot_index, right); // 将基准值放到数组最右边
        let mut store_index = left; // 记录小于等于基准值的元素位置

        // 遍历数组，将小于等于基准值的元素放到左边，大于基准值的元素放到右边
        for i in left..right {
            if nums[i] <= pivot {
                nums.swap(store_index, i);
                store_index += 1;
            }
        }

        nums.swap(store_index, right); // 将基准值放到正确的位置
        store_index // 返回基准值的位置
    }

    // 快速选择函数，返回数组中第k小的元素
    fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k_smallest: usize) -> i32 {
        if left == right {
            // 数组只有一个元素时，直接返回该元素
            return nums[left];
        }
        let mut rng = rand::thread_rng(); // 随机数生成器
        let mut pivot_index = left + rng.gen_range(0..right - left); // 随机选择基准值的位置
        pivot_index = Self::partition(nums, left, right, pivot_index); // 划分数组，返回基准值的位置
        return if k_smallest == pivot_index {
            // 如果基准值的位置正好是第k小的元素，直接返回该元素
            nums[k_smallest]
        } else if k_smallest < pivot_index {
            // 如果第k小的元素在基准值左边，继续在左边递归查找
            Self::quick_select(nums, left, pivot_index - 1, k_smallest)
        } else {
            // 如果第k小的元素在基准值右边，继续在右边递归查找
            Self::quick_select(nums, pivot_index + 1, right, k_smallest)
        };
    }

    // 主函数，返回数组中第k大的元素
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len(); // 数组长度
        Self::quick_select(&mut nums, 0, len - 1, len - k as usize) // 返回第len-k+1小的元素，即第k大的元素
    }
}
