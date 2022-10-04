// 排序数组
// https://leetcode.cn/problems/sort-an-array/
// INLINE  ../../images/sort/sort_an_array.jpeg
// 解题思路：在README.md中列举了"基础排序算法"，时间复杂度较低的算法都可以用于本题的解答。
// 这里实现的是堆排序，因测试快速排序按最后一个元素下标作为基准值未能通过LeetCode的测试（超时，未测试随机选择）。
// 按照目前我测试的结果，在Rust中，单线程顺序排序，完全随机和近似有序的情况下，堆排序和并归排序总体优于快速排序。

pub struct Solution;

impl Solution {
    pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len <= 1 {
            return nums;
        }

        Self::merge_sort_recursion(&mut nums, 0, len - 1);

        nums
    }

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

    fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let middle = (left + right) >> 1;
        Self::merge_sort_recursion(nums, left, middle);
        Self::merge_sort_recursion(nums, middle + 1, right);

        Self::merge(nums, left, middle, right);
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(nums)
    }
}

// 快排，未能通过测试
// impl Solution {
//     pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
//         let len = nums.len();
//         if len <= 1 {
//             return nums;
//         }
//         Self::quick_sort_recursion(&mut nums, 0, len - 1);
//         nums
//     }

//     fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
//         let pivot = nums[right]; // 基准值。最好选择随机一个元素，否则可能碰到极端情况。

//         let mut i = left;
//         // 默认不包含right，因此pivot取right更方便
//         for j in left..right {
//             if nums[j] < pivot {
//                 nums.swap(i, j);
//                 i += 1;
//             }
//         }

//         nums.swap(right, i);
//         i
//     }

//     fn quick_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
//         if left >= right {
//             return;
//         }

//         let pivot = Self::partition(nums, left, right);
//         if pivot != 0 {
//             Self::quick_sort_recursion(nums, left, pivot - 1);
//         }
//         Self::quick_sort_recursion(nums, pivot + 1, right);
//     }

//     pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
//         Self::quick_sort(nums)
//     }
// }
