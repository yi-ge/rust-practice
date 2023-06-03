// 重新格式化电话号码
// https://leetcode.cn/problems/reformat-phone-number
// INLINE  ../../images/string/reformat_phone_number.jpeg
// 解题思路：参考 https://leetcode.cn/problems/reformat-phone-number/solution/si-ge-yi-zu-yi-bian-chu-li-by-xavier-54-k9ri/

pub struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut res = Vec::<char>::new(); // 用于存储最终结果的字符数组
        let mut buff = Vec::<char>::with_capacity(4); // 用于存储每个数字组成的缓冲区，初始容量为4
        for ch in number.chars() {
            // 遍历输入的字符串
            if matches!(ch, '-' | ' ') {
                // 如果当前字符是'-'或者' '，则跳过
                continue;
            }

            if buff.len() == 4 {
                // 如果缓冲区中已经有4个数字了
                res.extend(&buff[..3]); // 将前三个数字添加到结果中
                res.push('-'); // 添加'-'字符
                let tmp = buff[3]; // 记录第四个数字
                buff.clear(); // 清空缓冲区
                buff.push(tmp); // 将第四个数字添加到缓冲区
            }
            buff.push(ch); // 将当前数字添加到缓冲区
        }

        if buff.len() < 4 {
            // 如果缓冲区中不足4个数字
            res.extend(&buff[..]); // 将缓冲区中所有数字添加到结果中
        } else {
            // 否则
            res.extend(&buff[..2]); // 将前两个数字添加到结果中
            res.push('-'); // 添加'-'字符
            res.extend(&buff[2..]); // 将后面的数字添加到结果中
        }

        res.iter().collect() // 将字符数组转换为字符串并返回
    }
}
