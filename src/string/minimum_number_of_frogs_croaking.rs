// 数青蛙
// https://leetcode.cn/problems/minimum-number-of-frogs-croaking
// INLINE  ../../images/string/minimum_number_of_frogs_croaking.jpeg

pub struct Solution;

impl Solution {
    // 输入一个字符串，表示一串青蛙叫声，返回最少需要多少只青蛙同时叫才能完成这段叫声
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        // 定义一个长度为5的数组，分别表示五种字母c,r,o,a,k出现的次数
        let mut frogs = vec![0; 5];
        // 最大青蛙数量
        let mut max_frogs = 0;
        // 遍历字符串中的每个字符
        for c in croak_of_frogs.chars() {
            // 根据不同的字符更新对应字母出现的次数
            let index = match c {
                'c' => 0,
                'r' => 1,
                'o' => 2,
                'a' => 3,
                'k' => 4,
                _ => panic!("invalid char"), // 如果出现了非法字符，直接panic
            };
            frogs[index] += 1;
            // 判断当前字母出现的次数是否大于前一个字母出现的次数，如果是则无法完成叫声，返回-1
            if index > 0 && frogs[index] > frogs[index - 1] {
                return -1;
            }
            // 更新最大青蛙数量
            max_frogs = max_frogs.max(frogs[0] - frogs[4]);
        }
        // 如果五种字母出现的次数相等，则返回最大青蛙数量，否则返回-1
        if frogs[0] == frogs[1]
            && frogs[1] == frogs[2]
            && frogs[2] == frogs[3]
            && frogs[3] == frogs[4]
        {
            max_frogs
        } else {
            -1
        }
    }
}
