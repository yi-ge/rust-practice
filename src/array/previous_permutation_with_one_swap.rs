// 交换一次的先前排列
// https://leetcode.cn/problems/previous-permutation-with-one-swap
// INLINE  ../../images/array/previous_permutation_with_one_swap.jpeg

pub struct Solution;

impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut new_arr = arr.clone();
        let len = new_arr.len();
        let mut cur_max = i32::MIN;
        let mut index = -1;
        let mut has_result = false;

        if len == 0 {
            return new_arr;
        }

        for i in (0..len - 1).rev() {
            if new_arr[i + 1] < new_arr[i] {
                for j in i + 1..len {
                    if new_arr[i] > new_arr[j] && new_arr[j] > cur_max {
                        has_result = true;
                        cur_max = new_arr[j];
                        index = j as i32;
                    }
                }
                if has_result {
                    new_arr.swap(i, index as usize);
                    return new_arr;
                }
            }
        }
        new_arr
    }
}
