// 通过翻转子数组使两个数组相等
// https://leetcode.cn/problems/make-two-arrays-equal-by-reversing-sub-arrays
// INLINE  ../../images/array/make_two_arrays_equal_by_reversing_sub_arrays.jpeg

pub struct Solution {}

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort();
        arr.sort();
        target == arr
    }
}
