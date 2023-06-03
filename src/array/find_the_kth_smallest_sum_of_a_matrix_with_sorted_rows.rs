// 有序矩阵中的第 k 个最小数组和
// https://leetcode.cn/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows
// INLINE  ../../images/array/find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows.jpeg

pub struct Solution;

impl Solution {
    // 获取数组中前k小的元素
    fn get_kth_smallest(mut nums: Vec<i32>, k: usize) -> Vec<i32> {
        nums.sort(); // 对数组进行排序
        nums.truncate(k); // 截取前k个元素
        nums // 返回结果数组
    }
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut res = vec![0]; // 初始化结果数组
        for row in mat {
            // 遍历每一行
            let mut tmp = vec![]; // 初始化临时数组
            for i in &res {
                // 遍历结果数组
                for j in &row {
                    // 遍历当前行
                    tmp.push(i + j); // 将结果数组中的元素与当前行中的元素相加，并加入临时数组
                }
            }
            res = Self::get_kth_smallest(tmp, k as usize); // 对临时数组获取前k小的元素，更新结果数组
        }
        res[k as usize - 1] // 返回结果数组中第k小的元素
    }
}
