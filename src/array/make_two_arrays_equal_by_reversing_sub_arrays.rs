// 通过翻转子数组使两个数组相等
// https://leetcode.cn/problems/make-two-arrays-equal-by-reversing-sub-arrays
// INLINE  ../../images/array/make_two_arrays_equal_by_reversing_sub_arrays.jpeg

pub struct Solution;

impl Solution {
    // 判断两个数组是否可以通过翻转子数组得到相等的结果
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        // 对两个数组进行排序
        target.sort();
        arr.sort();
        // 判断排序后的两个数组是否相等
        target == arr
    }
}
