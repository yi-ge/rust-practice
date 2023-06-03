// 交换一次的先前排列
// https://leetcode.cn/problems/previous-permutation-with-one-swap
// INLINE  ../../images/array/previous_permutation_with_one_swap.jpeg

pub struct Solution;

impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        // 复制原数组
        let mut new_arr = arr.clone();
        // 获取数组长度
        let len = new_arr.len();
        // 初始化当前最大值为最小值
        let mut cur_max = i32::MIN;
        // 初始化交换位置为-1
        let mut index = -1;
        // 标记是否有结果
        let mut has_result = false;

        // 如果数组长度为0，直接返回
        if len == 0 {
            return new_arr;
        }

        // 从后往前遍历数组
        for i in (0..len - 1).rev() {
            // 如果后一个元素比前一个元素小
            if new_arr[i + 1] < new_arr[i] {
                // 遍历后面的元素
                for j in i + 1..len {
                    // 如果当前元素比前一个元素小，且比当前最大值大
                    if new_arr[i] > new_arr[j] && new_arr[j] > cur_max {
                        // 标记有结果
                        has_result = true;
                        // 更新当前最大值和交换位置
                        cur_max = new_arr[j];
                        index = j as i32;
                    }
                }
                // 如果有结果，交换位置并返回新数组
                if has_result {
                    new_arr.swap(i, index as usize);
                    return new_arr;
                }
            }
        }
        // 没有结果，返回原数组
        new_arr
    }
}
