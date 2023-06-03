// 删除最短的子数组使剩余数组有序
// https://leetcode.cn/problems/shortest-subarray-to-be-removed-to-make-array-sorted
// INLINE  ../../images/stack/shortest_subarray_to_be_removed_to_make_array_sorted.jpeg

pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len(); // 数组长度
        let mut j = n - 1; // 定义指针 j 指向数组末尾
        while j > 0 && arr[j - 1] <= arr[j] {
            // 从数组末尾开始向前找到第一个不满足升序的位置
            j -= 1;
        }
        if j == 0 {
            // 如果整个数组已经升序，则不需要删除任何元素
            return 0;
        }
        let mut res = j; // 定义 res 为删除最短子数组后的剩余数组长度
        for i in 0..n {
            // 从数组开头开始遍历
            while j < n && arr[j] < arr[i] {
                // 找到 j 指向的第一个大于等于 arr[i] 的位置
                j += 1;
            }
            res = std::cmp::min(res, j - i - 1); // 更新 res，取最小值
            if i + 1 < n && arr[i] > arr[i + 1] {
                // 如果 j 指向的位置已经到达了数组结尾，或者 arr[i] 大于 arr[i+1]，则直接退出循环
                break;
            }
        }
        res as i32 // 返回最短子数组长度
    }
}
