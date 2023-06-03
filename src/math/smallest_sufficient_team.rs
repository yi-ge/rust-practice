// 最小的必要团队
// https://leetcode.cn/problems/smallest-sufficient-team
// INLINE  ../../images/math/smallest_sufficient_team.jpeg
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        // 获取所需技能和人员数量
        let n = req_skills.len();
        let m = people.len();
        // 构建技能到索引的映射
        let mut skill_index: HashMap<String, usize> = HashMap::new();
        for i in 0..n {
            skill_index.insert(req_skills[i].clone(), i);
        }
        // 初始化dp数组
        let mut dp = vec![m; 1 << n];
        dp[0] = 0;
        // 记录每个状态的前一个状态和选中的人员
        let mut prev_skill = vec![0; 1 << n];
        let mut prev_people = vec![0; 1 << n];
        // 遍历所有人员
        for i in 0..m {
            let mut cur_skill = 0;
            // 获取当前人员拥有的技能
            for skill in &people[i] {
                cur_skill |= 1 << skill_index[skill];
            }
            // 遍历所有状态
            for prev in 0..(1 << n) {
                let comb = prev | cur_skill;
                // 如果可以更新状态
                if dp[comb] > dp[prev] + 1 {
                    dp[comb] = dp[prev] + 1;
                    prev_skill[comb] = prev;
                    prev_people[comb] = i;
                }
            }
        }
        // 获取最小团队
        let mut res = Vec::new();
        let mut i = (1 << n) - 1;
        while i > 0 {
            res.push(prev_people[i] as i32);
            i = prev_skill[i];
        }
        res
    }
}
