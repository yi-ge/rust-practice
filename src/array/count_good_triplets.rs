// 统计好三元组
// https://leetcode.cn/problems/count-good-triplets/
// INLINE  ../../images/array/count_good_triplets.jpeg

pub struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len(); // 数组长度
        let mut res = 0; // 统计好三元组的数量
        for i in 0..n {
            // 第一个数
            for j in i + 1..n {
                // 第二个数
                for k in j + 1..n {
                    // 第三个数
                    if (arr[i] - arr[j]).abs() <= a // 判断第一个数和第二个数的差的绝对值是否小于等于 a
                        && (arr[j] - arr[k]).abs() <= b // 判断第二个数和第三个数的差的绝对值是否小于等于 b
                        && (arr[i] - arr[k]).abs() <= c
                    // 判断第一个数和第三个数的差的绝对值是否小于等于 c
                    {
                        res += 1; // 符合条件，好三元组数量加 1
                    }
                }
            }
        }
        res // 返回好三元组数量
    }
}
