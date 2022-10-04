// 统计好三元组
// https://leetcode.cn/problems/count-good-triplets/
// INLINE  ../../images/array/count_good_triplets.jpeg

pub struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len();
        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if (arr[i] - arr[j]).abs() <= a
                        && (arr[j] - arr[k]).abs() <= b
                        && (arr[i] - arr[k]).abs() <= c
                    {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}
