// 阶乘函数后 K 个零
// https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function
// INLINE  ../../images/math/preimage_size_of_factorial_zeroes_function.jpeg

pub struct Solution;

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        fn zeta(mut n: i64) -> i64 {
            let mut res = 0;
            while n > 0 {
                res += n / 5;
                n /= 5;
            }
            res
        }

        fn search(k: i64) -> i32 {
            let mut i = 0;
            let mut j = k * 5;
            while i < j {
                let m = (i + j) >> 1;
                let z = zeta(m);
                if z < k {
                    i = m + 1;
                } else {
                    j = m;
                }
            }

            i as i32
        }

        let k = k as i64;
        search(k + 1) - search(k)
    }
}
