// 公因子的数目
// https://leetcode.cn/problems/number-of-common-factors
// INLINE  ../../images/math/number_of_common_factors.jpeg

pub struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut count = 0; // 统计公因子数量的变量
        let mut i = 1; // 从1开始遍历a的因数
        while i * i <= a {
            // 遍历a的因数时，只需遍历到a的平方根即可
            if a % i == 0 {
                // 如果i是a的因数
                if b % i == 0 {
                    // 如果i也是b的因数
                    count += 1; // 公因子数量加1
                }
                if b % (a / i) == 0 && a / i != i {
                    // 如果a/i是a的因数，且不等于i
                    count += 1; // 公因子数量加1
                }
            }
            i += 1; // 遍历下一个可能的因数
        }
        count // 返回公因子数量
    }
}
