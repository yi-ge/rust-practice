// 判定是否互为字符重排
// https://leetcode.cn/problems/check-permutation-lcci
// INLINE  ../../images/map/check_permutation_lcci.jpeg

pub struct Solution;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            // 如果两个字符串长度不相等，则肯定不是互为字符重排
            return false;
        }

        let mut xor = 0u32; // 定义按位异或变量，用于判断两个字符串是否互为字符重排
        let mut ascii_sum1 = 0u32; // 定义ASCII求和变量，用于排除两个字符串内部成对
        let mut ascii_sum2 = 0u32;
        let (s1_arr, s2_arr) = (s1.as_bytes().to_vec(), s2.as_bytes().to_vec()); // 将两个字符串转换为字节数组
        for i in 0..s1.len() {
            xor ^= s1_arr[i] as u32 ^ s2_arr[i] as u32; // 逐位进行异或操作，相同得0，不同得1
            ascii_sum1 += s1_arr[i] as u32; // 计算第一个字符串的ASCII码总和
            ascii_sum2 += s2_arr[i] as u32; // 计算第二个字符串的ASCII码总和
        }

        xor == 0 && ascii_sum1 == ascii_sum2 // 如果按位异或结果为0且两个字符串ASCII码总和相等，则两个字符串互为字符重排
    }
}
