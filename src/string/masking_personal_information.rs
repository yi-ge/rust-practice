// 隐藏个人信息
// https://leetcode.cn/problems/masking-personal-information
// INLINE  ../../images/string/masking_personal_information.jpeg

pub struct Solution;

impl Solution {
    fn mask_email(s: &str) -> String {
        let s = s.to_ascii_lowercase();
        let at = s.find('@').unwrap();
        format!("{}*****{}", &s[0..1], &s[at - 1..])
    }

    fn mask_phone(s: &str) -> String {
        let country = vec!["", "+*-", "+**-", "+***-"];
        let s: String = s.chars().filter(|c| c.is_numeric()).collect();
        format!("{}***-***-{}", country[s.len() - 10], &s[s.len() - 4..])
    }

    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            Self::mask_email(&s)
        } else {
            Self::mask_phone(&s)
        }
    }
}
