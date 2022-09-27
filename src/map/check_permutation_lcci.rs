// 判定是否互为字符重排
// https://leetcode.cn/problems/check-permutation-lcci
// INLINE  ../../images/map/check_permutation_lcci.jpeg

pub struct Solution;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        let mut xor = 0u32; // 按位异或，相同得0，不同得1
        let mut ascii_sum1 = 0u32; // ASCII求和，排除两个字符串内部成对
        let mut ascii_sum2 = 0u32;
        let (s1_arr, s2_arr) = (s1.as_bytes().to_vec(), s2.as_bytes().to_vec());
        for i in 0..s1.len() {
            xor ^= s1_arr[i] as u32 ^ s2_arr[i] as u32;
            ascii_sum1 += s1_arr[i] as u32;
            ascii_sum2 += s2_arr[i] as u32;
        }

        xor == 0 && ascii_sum1 == ascii_sum2
    }
}
