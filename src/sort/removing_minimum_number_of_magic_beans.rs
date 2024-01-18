// 拿出最少数目的魔法豆
// https://leetcode.cn/problems/removing-minimum-number-of-magic-beans
// INLINE  ../../images/sort/removing_minimum_number_of_magic_beans.jpeg

pub struct Solution;

impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let n = beans.len();
        let mut sorted_beans = beans.clone();
        sorted_beans.sort();

        let total: i64 = sorted_beans.iter().map(|&x| x as i64).sum();
        let mut res = total;

        for i in 0..n {
            res = res.min(total - (sorted_beans[i] as i64) * (n - i) as i64);
        }

        res
    }
}
