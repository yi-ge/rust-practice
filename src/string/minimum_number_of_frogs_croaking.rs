// 数青蛙
// https://leetcode.cn/problems/minimum-number-of-frogs-croaking
// INLINE  ../../images/string/minimum_number_of_frogs_croaking.jpeg

pub struct Solution;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut frogs = vec![0; 5];
        let mut max_frogs = 0;
        for c in croak_of_frogs.chars() {
            let index = match c {
                'c' => 0,
                'r' => 1,
                'o' => 2,
                'a' => 3,
                'k' => 4,
                _ => panic!("invalid char"),
            };
            frogs[index] += 1;
            if index > 0 && frogs[index] > frogs[index - 1] {
                return -1;
            }
            max_frogs = max_frogs.max(frogs[0] - frogs[4]);
        }
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
