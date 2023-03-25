// 删除最短的子数组使剩余数组有序
// https://leetcode.cn/problems/shortest-subarray-to-be-removed-to-make-array-sorted
// INLINE  ../../images/stack/shortest_subarray_to_be_removed_to_make_array_sorted.jpeg

pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut j = n - 1;
        while j > 0 && arr[j - 1] <= arr[j] {
            j -= 1;
        }
        if j == 0 {
            return 0;
        }
        let mut res = j;
        for i in 0..n {
            while j < n && arr[j] < arr[i] {
                j += 1;
            }
            res = std::cmp::min(res, j - i - 1);
            if i + 1 < n && arr[i] > arr[i + 1] {
                break;
            }
        }
        res as i32
    }
}
