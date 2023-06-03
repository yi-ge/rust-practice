// 阶乘函数后 K 个零
// https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function
// INLINE  ../../images/math/preimage_size_of_factorial_zeroes_function.jpeg

pub struct Solution;

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        // 计算 n! 中因子 5 的个数
        fn zeta(mut n: i64) -> i64 {
            let mut res = 0;
            while n > 0 {
                res += n / 5; // 每个因子 5 贡献一个因子 5 的个数
                n /= 5; // 继续向下计算
            }
            res // 返回 n! 中因子 5 的个数
        }

        // 二分查找满足 zeta(m) == k 的最小的 m
        fn search(k: i64) -> i32 {
            let mut i = 0;
            let mut j = k * 5;
            while i < j {
                let m = (i + j) >> 1; // 计算中间位置
                let z = zeta(m); // 计算 m! 中因子 5 的个数
                if z < k {
                    // 如果 z 小于 k，说明 m 不满足条件，继续在右半部分查找
                    i = m + 1;
                } else {
                    // 否则，在左半部分查找
                    j = m;
                }
            }

            i as i32 // 返回最小的 m
        }

        let k = k as i64;
        search(k + 1) - search(k) // 返回满足条件的数的个数
    }
}
