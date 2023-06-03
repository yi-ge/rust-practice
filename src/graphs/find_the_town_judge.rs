// 找到小镇的法官
// https://leetcode.cn/problems/find-the-town-judge
// INLINE  ../../images/graphs/find_the_town_judge.jpeg

pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        // 初始化入度和出度数组
        let mut in_degrees: Vec<i32> = vec![0; (n + 1) as usize];
        let mut out_degrees: Vec<i32> = vec![0; (n + 1) as usize];

        // 遍历trust数组，计算每个人的入度和出度
        trust.iter().for_each(|v| {
            in_degrees[v[1] as usize] += 1;
            out_degrees[v[0] as usize] += 1;
        });

        // 遍历每个人，如果入度为n-1且出度为0，说明该人是法官
        for i in 1..n + 1 {
            if in_degrees[i as usize] == n - 1 && out_degrees[i as usize] == 0 {
                return i;
            }
        }

        // 没有找到法官，返回-1
        -1
    }
}
