// 隐藏个人信息
// https://leetcode.cn/problems/masking-personal-information
// INLINE  ../../images/string/masking_personal_information.jpeg

pub struct Solution;

impl Solution {
    // 针对电子邮件的掩码函数
    fn mask_email(s: &str) -> String {
        let s = s.to_ascii_lowercase(); // 将字符串转为小写字母
        let at = s.find('@').unwrap(); // 找到 @ 符号的位置
        format!("{}*****{}", &s[0..1], &s[at - 1..]) // 返回掩码后的字符串，只显示首尾字符，中间用 * 号代替
    }

    // 针对电话号码的掩码函数
    fn mask_phone(s: &str) -> String {
        let country = vec!["", "+*-", "+**-", "+***-"]; // 国家代码
        let s: String = s.chars().filter(|c| c.is_numeric()).collect(); // 将字符串中的数字提取出来
        format!("{}***-***-{}", country[s.len() - 10], &s[s.len() - 4..]) // 返回掩码后的字符串，显示国家代码和末尾 4 个数字，中间用 * 号代替
    }

    // 统一的掩码函数，根据字符串中是否包含 @ 判断是电子邮件还是电话号码
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            // 如果字符串中包含 @ 符号，说明是电子邮件
            Self::mask_email(&s) // 调用针对电子邮件的掩码函数
        } else {
            // 否则是电话号码
            Self::mask_phone(&s) // 调用针对电话号码的掩码函数
        }
    }
}
