// 有序矩阵中的第 k 个最小数组和
// https://leetcode.cn/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows
// INLINE  ../../images/array/find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows.jpeg

pub struct Solution;

impl Solution {
    fn get_kth_smallest(mut nums: Vec<i32>, k: usize) -> Vec<i32> {
        nums.sort();
        nums.truncate(k);
        nums
    }
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut res = vec![0];
        for row in mat {
            let mut tmp = vec![];
            for i in &res {
                for j in &row {
                    tmp.push(i + j);
                }
            }
            res = Self::get_kth_smallest(tmp, k as usize);
        }
        res[k as usize - 1]
    }
}
