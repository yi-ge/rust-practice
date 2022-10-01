// 重新格式化电话号码
// https://leetcode.cn/problems/reformat-phone-number
// INLINE  ../../images/string/reformat_phone_number.jpeg
// 解题思路：参考 https://leetcode.cn/problems/reformat-phone-number/solution/si-ge-yi-zu-yi-bian-chu-li-by-xavier-54-k9ri/

pub struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut res = Vec::<char>::new();
        let mut buff = Vec::<char>::with_capacity(4);
        for ch in number.chars() {
            if matches!(ch, '-' | ' ') {
                continue;
            }

            if buff.len() == 4 {
                res.extend(&buff[..3]);
                res.push('-');
                let tmp = buff[3];
                buff.clear();
                buff.push(tmp);
            }
            buff.push(ch);
        }

        if buff.len() < 4 {
            res.extend(&buff[..]);
        } else {
            res.extend(&buff[..2]);
            res.push('-');
            res.extend(&buff[2..]);
        }

        res.iter().collect()
    }
}
